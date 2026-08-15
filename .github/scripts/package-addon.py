#!/usr/bin/env python3
"""Package a WoW addon directory into a CurseForge-ready zip.

Usage:
    package-addon.py <addon-dir> <version> [--out-dir DIR]

The zip root is the addon folder name itself (e.g. JojaAutoPetter/JojaAutoPetter.toc).
Version is injected into every .toc file's ## Version: line in the *packaged* copy
(source files are not modified). Dotfiles/dot-dirs are excluded.
"""

import argparse
import os
import re
import sys
import zipfile


def collect_files(addon_dir):
    """Return sorted list of relative paths (from parent of addon_dir), excluding dotfiles."""
    base = os.path.dirname(addon_dir)
    folder = os.path.basename(addon_dir)
    result = []
    for root, dirs, files in os.walk(addon_dir):
        dirs[:] = sorted(d for d in dirs if not d.startswith("."))
        for f in sorted(files):
            if f.startswith("."):
                continue
            full = os.path.join(root, f)
            rel = os.path.relpath(full, base)
            result.append(rel)
    return result


def inject_version(content, version):
    if re.search(r"^## Version:", content, re.MULTILINE):
        return re.sub(
            r"^(## Version:).*$",
            rf"\1 {version}",
            content,
            count=1,
            flags=re.MULTILINE,
        )
    return re.sub(
        r"^(## Interface:.*(?:\r?\n))",
        rf"\1## Version: {version}\n",
        content,
        count=1,
        flags=re.MULTILINE,
    )


def build_zip(addon_dir, version, out_dir):
    addon_dir = os.path.normpath(addon_dir)
    name = os.path.basename(addon_dir)
    zip_name = f"{name}-{version}.zip"
    zip_path = os.path.join(out_dir, zip_name)

    files = collect_files(addon_dir)
    if not files:
        print(f"ERROR: no files found in {addon_dir}", file=sys.stderr)
        return None

    base = os.path.dirname(addon_dir)
    with zipfile.ZipFile(zip_path, "w", zipfile.ZIP_DEFLATED) as zf:
        for rel in files:
            full = os.path.join(base, rel)
            if rel.endswith(".toc"):
                with open(full, "r") as fh:
                    data = inject_version(fh.read(), version)
                zf.writestr(rel, data)
            else:
                zf.write(full, rel)

    return zip_path


def main():
    parser = argparse.ArgumentParser(description="Package a WoW addon for CurseForge.")
    parser.add_argument("addon_dir", help="Path to the addon directory")
    parser.add_argument("version", help="Version string to inject (e.g. 1.0.42)")
    parser.add_argument("--out-dir", default=".", help="Output directory for the zip")
    args = parser.parse_args()

    if not os.path.isdir(args.addon_dir):
        print(f"ERROR: {args.addon_dir} is not a directory", file=sys.stderr)
        sys.exit(1)

    os.makedirs(args.out_dir, exist_ok=True)
    zip_path = build_zip(args.addon_dir, args.version, args.out_dir)
    if not zip_path:
        sys.exit(1)
    print(zip_path)


if __name__ == "__main__":
    main()
