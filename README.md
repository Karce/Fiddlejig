# Fiddlejig

A collection of player-aiding tools — **macros** and **addons** — for
**World of Warcraft Classic: The Burning Crusade Anniversary** (WoW Classic 2.5.x).

Fiddlejig is a grab-bag of quality-of-life and automation helpers: one-click
rotation aids, smart utility macros, and small addons for things a macro can't
quite reach. It's an undead-themed name fitting for tools that do the player's
busywork for them.

## Philosophy: macros first, addons as a fallback

Macros are lightweight, share-able, and need no installation beyond pasting them
in. So **every feature starts as a macro**. We only build a full addon when a
macro genuinely can't do the job — when it needs persistent state, event
handling, a UI, or logic past the 255-character macro limit.

## What's here (planned & in progress)

- **Hunter pet feeding** — feed your pet the best available food in your bags.
- **Rotation helpers** — one-click / castsequence macros to smooth out rotations.
- ...more to come. See [`TODO.md`](TODO.md) for the working feature list.

## Repository layout

```
Fiddlejig/
├── macros/        # Macro collection, grouped by class/category
│   └── <category>/<name>.md   # description + macro text + usage notes
├── addons/        # Full addons (fallback when a macro won't do)
│   └── <AddonName>/<AddonName>.toc + Lua sources
├── docs/          # Reference & conventions
│   ├── api/       # WoW Classic TBC Anniversary (2.5.x) Lua API reference
│   └── conventions.md
├── config/        # Local config (reference, not code)
│   ├── paths.example.md    # template (committed)
│   └── paths.local.md      # your local game paths (gitignored)
├── .luarc.json    # Lua Language Server config (Lua 5.1 + WoW API)
├── .vscode/       # Recommended extensions (extensions.json is shared)
├── scripts/       # Helper scripts (setup, install/sync, etc.)
└── TODO.md        # Working feature backlog
```

> AI-assist rules live in a local-only `CLAUDE.local.md` (gitignored). Human-facing
> conventions are in [`docs/conventions.md`](docs/conventions.md).

## Getting started

1. Copy the config template and fill in your local paths:
   ```sh
   cp config/paths.example.md config/paths.local.md
   ```
   `config/paths.local.md` is gitignored — your machine-specific paths never get
   committed.
2. Browse [`macros/`](macros/) and copy a macro into the game's macro UI
   (`/macro`), or install an addon by copying its folder into your
   `Interface/AddOns` directory.

## Using the macros

WoW macros are plain text, **limited to 255 characters**. Each macro file
documents what it does and includes the macro body in a code fence — copy that
body into the in-game macro editor.

## Using the addons

Copy an addon folder from `addons/` into your WoW
`_classic_/Interface/AddOns/` directory, then enable it on the character-select
addons screen.

## Contributing / development

See [`docs/conventions.md`](docs/conventions.md) for project conventions. Note this
is a **public** repository — never commit local paths, usernames, or any personal
information.

## License

[GPL-3.0](LICENSE).
