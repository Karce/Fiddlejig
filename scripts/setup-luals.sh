#!/usr/bin/env bash
# Fetch WoW API EmmyLua/LuaLS annotations into .luals/wow-api (gitignored).
#
# These annotations give the Lua Language Server (sumneko) autocomplete and type
# checking for the WoW API. Works with any LuaLS-based editor (VS Code, Neovim,
# Zed). `.luarc.json` already points workspace.library at .luals/wow-api/Annotations.
#
# VS Code users can alternatively just install the "WoW API" extension
# (ketho.wow-api), which auto-loads these when you open a folder with a .toc.
#
# Usage:  scripts/setup-luals.sh
set -euo pipefail

REPO="https://github.com/Ketho/vscode-wow-api.git"
DEST=".luals/wow-api"

cd "$(dirname "$0")/.."

if [ -d "$DEST/.git" ]; then
  echo "Updating existing annotations in $DEST ..."
  git -C "$DEST" pull --ff-only
else
  echo "Cloning WoW API annotations into $DEST ..."
  mkdir -p "$(dirname "$DEST")"
  git clone --depth 1 "$REPO" "$DEST"
fi

if [ -d "$DEST/Annotations" ]; then
  echo "Done. Annotations available at $DEST/Annotations"
else
  echo "WARNING: $DEST/Annotations not found — check the repo layout and update"
  echo "         workspace.library in .luarc.json accordingly."
fi
