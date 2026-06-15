# Training the NN bobber detector

This is the offline pipeline that produces `bot/models/bobber.onnx` — the learned
replacement for the Haar cascade. The Rust bot loads the exported ONNX via `tract`
(see the `Detector` trait and roadmap in the main README). Everything here runs
outside the Rust build; the generated dataset and training runs are gitignored.

The detector's target is the **splashing bobber** (the bite) — the same thing the
cascade detected — so the existing box labels are reused as-is.

## 0. Prerequisites

```sh
python3 -m venv .venv && source .venv/bin/activate
pip install ultralytics onnx onnxslim         # ultralytics pulls in torch + Pillow
```

A GPU makes training minutes instead of an hour, but this corpus is tiny enough to
train on CPU.

## 1. Build the YOLO dataset

```sh
python3 training/nn/convert_labels.py          # symlinks images; --copy to duplicate
```

Converts `training/pos.txt` (one splash box per image, OpenCV `path num x y w h`)
and `training/neg.txt` into an Ultralytics dataset under `training/nn/dataset/`,
**zone-stratified** 80/20 so val isn't all Darkshore. Negatives become empty label
files (explicit background → fewer false positives). Prints a per-zone train/val
table. Re-run any time; it rebuilds from scratch.

## 2. Train (transfer-learn from COCO)

The corpus is small (136 positives, 72% Darkshore), so **always start from
pretrained weights** and lean on augmentation — never train from scratch.

```sh
yolo detect train model=yolo11n.pt data=training/nn/dataset/data.yaml \
  imgsz=960 epochs=120 patience=30 batch=8 \
  degrees=0 fliplr=0.5 mosaic=1.0 close_mosaic=15 \
  project=training/nn/runs name=bobber
```

### Why `imgsz=960` (the key knob)

The bobber is **~1.5–2% of frame width** (~50 px in a 3440-wide capture). After YOLO
resizes the image to a square input, that becomes:

| `imgsz` | bobber size in the model input | verdict |
|--------:|-------------------------------:|---------|
| 320     | ~5 px                          | too small to learn reliably |
| 640     | ~10 px                         | marginal |
| **960** | **~15 px**                     | **good starting point** |
| 1280    | ~19 px                         | best recall, slower |

Train and **infer at the same `imgsz`** so the bobber's apparent size matches
(`nn_input_size` in `bot/src/config.rs` must equal this value). Start at 960; bump to
1280 if recall on the low-sample zones (Redridge/Stranglethorn) is weak.

> **Cheaper-inference path (later optimization):** to run a small 320-input model,
> we'd train on **ROI crops** (~640 px regions around the bobber) instead of full
> frames, so the bobber is ~20 px at 320. That needs a `--crop` mode added to
> `convert_labels.py` and a matching inference ROI. Do this only if full-frame
> inference cost is still too high after Phase 0's fps cap — getting the model
> *robust* comes first.

## 3. Export to tract-safe ONNX

```sh
yolo export model=training/nn/runs/bobber/weights/best.pt format=onnx \
  imgsz=960 opset=12 simplify=True nms=False dynamic=False
```

- **`nms=False`** is essential — embedded NMS uses ops `tract` can't run; the Rust
  side does NMS itself.
- `opset=12` + `simplify=True` + `dynamic=False` maximize `tract` compatibility.
- Optionally open the result in [netron](https://netron.app) and confirm there's no
  `NonMaxSuppression` / `ScatterND` / `GridSample` node.

## 4. Deploy

```sh
cp training/nn/runs/bobber/weights/best.onnx bot/models/bobber.onnx
```

Then in `bot/src/config.rs` set `backend: Nn` and `nn_input_size` to the `imgsz` you
trained/exported with. The `.onnx` (~6–12 MB) is committed alongside the cascades.

## 5. Gather more data (improving robustness)

Per-zone recall is gated by per-zone samples (Redridge has only 8 positives). The
legacy capture tool in `training/` grabs more: press `f` on a splash frame, `d` on a
no-bite frame, annotate the new positives into `pos.txt` with `opencv_annotation`,
then re-run from step 1.
