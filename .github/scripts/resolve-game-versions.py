#!/usr/bin/env python3
"""Resolve WoW TOC interface numbers to CurseForge gameVersion IDs.

Usage:
    resolve-game-versions.py <toc-file> [<toc-file> ...]

Reads ## Interface: from each TOC, queries the CurseForge game versions API,
and prints a JSON array of matching numeric version IDs to stdout.

Requires CF_API_TOKEN in the environment.
"""

import json
import os
import re
import sys
import urllib.request


def parse_interface(toc_path):
    with open(toc_path) as f:
        for line in f:
            m = re.match(r"^## Interface:\s*(\d+)", line)
            if m:
                return m.group(1)
    return None


def fetch_game_versions(token):
    req = urllib.request.Request(
        "https://wow.curseforge.com/api/game/versions",
        headers={"X-Api-Token": token},
    )
    with urllib.request.urlopen(req) as resp:
        if resp.status != 200:
            print(f"ERROR: game versions API returned {resp.status}", file=sys.stderr)
            sys.exit(1)
        return json.loads(resp.read())


def main():
    if len(sys.argv) < 2:
        print("Usage: resolve-game-versions.py <toc> [<toc> ...]", file=sys.stderr)
        sys.exit(1)

    token = os.environ.get("CF_API_TOKEN")
    if not token:
        print("ERROR: CF_API_TOKEN not set", file=sys.stderr)
        sys.exit(1)

    interfaces = set()
    for toc in sys.argv[1:]:
        iface = parse_interface(toc)
        if iface:
            interfaces.add(iface)
        else:
            print(f"WARNING: no ## Interface: found in {toc}", file=sys.stderr)

    if not interfaces:
        print("ERROR: no interface numbers found", file=sys.stderr)
        sys.exit(1)

    versions = fetch_game_versions(token)
    ids = []
    for v in versions:
        if v.get("apiVersion") in interfaces:
            ids.append(v["id"])

    if not ids:
        print(
            f"ERROR: no CurseForge version IDs matched interfaces {interfaces}",
            file=sys.stderr,
        )
        sys.exit(1)

    print(json.dumps(sorted(ids)))


if __name__ == "__main__":
    main()
