#!/usr/bin/env python3
"""Convert the OpenCV cascade corpus into a YOLO detection dataset.

Reads ``training/pos.txt`` (one splash-bobber box per positive image, in OpenCV
``path num x y w h`` format with absolute top-left + width/height) and
``training/neg.txt`` (background images), and writes an Ultralytics-layout dataset
under ``training/nn/dataset/`` with a **zone-stratified** train/val split.

The positives are *bite / splash* frames — the same labels the Haar cascade was
trained on — so the model learns to fire on the splash, matching the cascade's
behaviour. Negatives become **empty** label files (explicit background examples,
which suppress false positives).

Paths are resolved relative to this file, so it runs from anywhere and hardcodes
no machine-specific path (this is a public repo). The generated ``dataset/`` —
including ``data.yaml``, which carries an absolute path — is gitignored.

Requires only Pillow (for image dimensions). Usage::

    python3 training/nn/convert_labels.py            # symlink images (default)
    python3 training/nn/convert_labels.py --copy     # copy instead of symlink
"""

from __future__ import annotations

import argparse
import os
import random
import shutil
import sys
from pathlib import Path

from PIL import Image

TRAINING_ROOT = Path(__file__).resolve().parent.parent  # tools/autofisher-3000/training
DATASET = Path(__file__).resolve().parent / "dataset"


def zone_of(rel_path: str) -> str:
    """`positive/darkshore/123.jpg` -> `darkshore`."""
    parts = Path(rel_path).parts
    return parts[1] if len(parts) >= 2 else "unknown"


def parse_positives(pos_txt: Path) -> list[tuple[str, str, tuple[int, int, int, int]]]:
    """Return (rel_path, zone, (x, y, w, h)) for each annotated positive."""
    items = []
    for line in pos_txt.read_text().splitlines():
        line = line.strip()
        if not line:
            continue
        fields = line.split()
        # `path num x y w h` — every line has exactly one box (num == 1)
        rel, num = fields[0], int(fields[1])
        if num != 1:
            print(f"  warn: {rel} has {num} boxes; only the first is used", file=sys.stderr)
        x, y, w, h = (int(v) for v in fields[2:6])
        items.append((rel, zone_of(rel), (x, y, w, h)))
    return items


def parse_negatives(neg_txt: Path) -> list[tuple[str, str]]:
    """Return (rel_path, zone) for each background image."""
    items = []
    for line in neg_txt.read_text().splitlines():
        line = line.strip()
        if line:
            items.append((line, zone_of(line)))
    return items


def stratified_split(keys: list, get_zone, val_frac: float, seed: int) -> set:
    """Pick a val subset, stratified per zone so val isn't all one zone.

    Returns the set of indices assigned to val. Each zone with >= 2 items
    contributes at least one val item (and always keeps at least one for train).
    """
    by_zone: dict[str, list[int]] = {}
    for i, k in enumerate(keys):
        by_zone.setdefault(get_zone(k), []).append(i)

    rng = random.Random(seed)
    val: set[int] = set()
    for zone, idxs in sorted(by_zone.items()):
        idxs = idxs[:]
        rng.shuffle(idxs)
        n = len(idxs)
        n_val = max(1, round(n * val_frac)) if n >= 2 else 0
        n_val = min(n_val, n - 1)  # always leave at least one for train
        val.update(idxs[:n_val])
    return val


def yolo_line(box: tuple[int, int, int, int], img_w: int, img_h: int) -> str:
    """OpenCV (x, y, w, h) top-left box -> normalized YOLO `0 cx cy w h`."""
    x, y, w, h = box
    cx = (x + w / 2) / img_w
    cy = (y + h / 2) / img_h
    nw = w / img_w
    nh = h / img_h
    clamp = lambda v: min(1.0, max(0.0, v))  # noqa: E731 — guards against off-by-edge boxes
    return f"0 {clamp(cx):.6f} {clamp(cy):.6f} {clamp(nw):.6f} {clamp(nh):.6f}\n"


def place_image(src: Path, dst: Path, copy: bool) -> None:
    if dst.exists() or dst.is_symlink():
        dst.unlink()
    if copy:
        shutil.copy2(src, dst)
    else:
        os.symlink(src.resolve(), dst)  # absolute target so the link is CWD-independent


def reset_dirs() -> dict[str, Path]:
    dirs = {
        "img_train": DATASET / "images" / "train",
        "img_val": DATASET / "images" / "val",
        "lbl_train": DATASET / "labels" / "train",
        "lbl_val": DATASET / "labels" / "val",
    }
    for sub in ("images", "labels"):
        shutil.rmtree(DATASET / sub, ignore_errors=True)
    for d in dirs.values():
        d.mkdir(parents=True, exist_ok=True)
    return dirs


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--val-frac", type=float, default=0.2, help="fraction held out for val")
    ap.add_argument("--seed", type=int, default=1337, help="split RNG seed (reproducible)")
    ap.add_argument("--copy", action="store_true", help="copy images instead of symlinking")
    args = ap.parse_args()

    pos = parse_positives(TRAINING_ROOT / "pos.txt")
    neg = parse_negatives(TRAINING_ROOT / "neg.txt")
    if not pos:
        print("no positives found — is the corpus present?", file=sys.stderr)
        return 1

    pos_val = stratified_split(pos, lambda p: p[1], args.val_frac, args.seed)
    neg_val = stratified_split(neg, lambda n: n[1], args.val_frac, args.seed)

    dirs = reset_dirs()
    counts: dict[str, dict[str, int]] = {}  # zone -> {"train": n, "val": n}
    missing = 0

    def record(zone: str, split: str) -> None:
        counts.setdefault(zone, {"train": 0, "val": 0})[split] += 1

    # positives: symlink image + write the YOLO box label
    for i, (rel, zone, box) in enumerate(pos):
        src = TRAINING_ROOT / rel
        if not src.exists():
            missing += 1
            continue
        split = "val" if i in pos_val else "train"
        stem = f"{zone}__pos__{Path(rel).stem}"
        place_image(src, dirs[f"img_{split}"] / f"{stem}{src.suffix}", args.copy)
        with Image.open(src) as im:
            w, h = im.size
        (dirs[f"lbl_{split}"] / f"{stem}.txt").write_text(yolo_line(box, w, h))
        record(zone, split)

    # negatives: symlink image + empty label (explicit background)
    for i, (rel, zone) in enumerate(neg):
        src = TRAINING_ROOT / rel
        if not src.exists():
            missing += 1
            continue
        split = "val" if i in neg_val else "train"
        stem = f"{zone}__neg__{Path(rel).stem}"
        place_image(src, dirs[f"img_{split}"] / f"{stem}{src.suffix}", args.copy)
        (dirs[f"lbl_{split}"] / f"{stem}.txt").write_text("")
        record(zone, split)

    data_yaml = DATASET / "data.yaml"
    data_yaml.write_text(
        f"# Generated by convert_labels.py — gitignored (carries an absolute path).\n"
        f"path: {DATASET.resolve()}\n"
        f"train: images/train\n"
        f"val: images/val\n"
        f"names:\n"
        f"  0: bobber\n"
    )

    print(f"dataset → {DATASET}")
    print(f"  positives: {len(pos)}  negatives: {len(neg)}  missing-on-disk: {missing}")
    print(f"  {'zone':<14}{'train':>7}{'val':>6}")
    for zone in sorted(counts):
        c = counts[zone]
        print(f"  {zone:<14}{c['train']:>7}{c['val']:>6}")
    print(f"  data.yaml → {data_yaml}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
