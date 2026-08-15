#!/usr/bin/env python3
"""Check Blizzard game versions and optionally update addon TOC Interface numbers.

Fetches latest build versions from wago.tools, compares against ## Interface:
values in addon TOC files, and optionally rewrites them in place.

Environment:
    REPO_ROOT       Repository root (default: .)
    GITHUB_OUTPUT   GitHub Actions output file
    _WAGO_OVERRIDE  JSON mapping product -> version string, bypasses API fetch
"""

import json
import os
import re
import sys
import urllib.request

VERSION_RE = re.compile(r"^\d+\.\d+\.\d+(\.\d+)?$")

PRODUCTS = {
    "wow_anniversary": ".toc",
    "wow_classic_era": "_Vanilla.toc",
}


def version_to_interface(ver):
    p = ver.split(".")
    return int(p[0]) * 10000 + int(p[1]) * 100 + int(p[2])


def find_tocs(repo_root):
    result = {k: [] for k in PRODUCTS}
    addons_dir = os.path.join(repo_root, "addons")
    if not os.path.isdir(addons_dir):
        return result
    for addon in sorted(os.listdir(addons_dir)):
        addon_path = os.path.join(addons_dir, addon)
        if not os.path.isdir(addon_path):
            continue
        for product, suffix in PRODUCTS.items():
            toc = os.path.join(addon_path, addon + suffix)
            if os.path.isfile(toc):
                result[product].append(toc)
    return result


def read_interface(path):
    with open(path) as f:
        for line in f:
            m = re.match(r"^## Interface:\s*(\d+)", line)
            if m:
                return int(m.group(1))
    raise ValueError(f"No ## Interface: in {path}")


def rewrite_interface(path, new_val):
    with open(path, "rb") as f:
        data = f.read()
    updated = re.sub(
        rb"^(## Interface:\s*)\d+",
        rb"\g<1>" + str(new_val).encode(),
        data,
        flags=re.MULTILINE,
    )
    with open(path, "wb") as f:
        f.write(updated)


def fetch_latest():
    override = os.environ.get("_WAGO_OVERRIDE")
    if override:
        versions = json.loads(override)
    else:
        req = urllib.request.Request(
            "https://wago.tools/api/builds",
            headers={"User-Agent": "Fiddlejig-CI/1.0 (game-version-check)"},
        )
        with urllib.request.urlopen(req, timeout=30) as resp:
            data = json.loads(resp.read())
        versions = {}
        for product in PRODUCTS:
            builds = data.get(product)
            if not builds:
                print(f"ERROR: no builds for {product}", file=sys.stderr)
                sys.exit(1)
            latest = max(builds, key=lambda b: b.get("created_at", ""))
            versions[product] = latest["version"]

    for product in PRODUCTS:
        if product not in versions:
            print(f"ERROR: missing product {product}", file=sys.stderr)
            sys.exit(1)

    for product, ver in versions.items():
        if not VERSION_RE.match(ver):
            print(f"ERROR: invalid version for {product}: {ver!r}", file=sys.stderr)
            sys.exit(1)

    return versions


def gh_output(key, value):
    path = os.environ.get("GITHUB_OUTPUT")
    if path:
        with open(path, "a") as f:
            f.write(f"{key}={value}\n")


def main():
    repo_root = os.environ.get("REPO_ROOT", ".")
    do_rewrite = "--rewrite" in sys.argv

    latest = fetch_latest()
    tocs = find_tocs(repo_root)

    changed = False
    products = {}

    for product in PRODUCTS:
        ver = latest[product]
        new_iface = version_to_interface(ver)
        paths = tocs[product]
        if not paths:
            continue
        old_iface = read_interface(paths[0])
        if old_iface != new_iface:
            changed = True
            if do_rewrite:
                for p in paths:
                    rewrite_interface(p, new_iface)
        products[product] = {
            "version": ver,
            "old_interface": old_iface,
            "new_interface": new_iface,
        }

    print(json.dumps({"changed": changed, "products": products}))

    gh_output("changed", str(changed).lower())
    if changed:
        ann = products["wow_anniversary"]
        era = products["wow_classic_era"]
        gh_output("branch", f"bump/interface-{ann['new_interface']}-{era['new_interface']}")
        for prefix, d in [("ann", ann), ("era", era)]:
            gh_output(f"{prefix}_old", d["old_interface"])
            gh_output(f"{prefix}_new", d["new_interface"])
            gh_output(f"{prefix}_ver", d["version"])

    return 0


if __name__ == "__main__":
    sys.exit(main())
