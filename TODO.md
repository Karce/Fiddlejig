# Fiddlejig — TODO

Working document for features and tasks. Newest context at the top of each
section. **Policy:** prefer a macro; build an addon only when a macro can't do it.

## Now

- [ ] **Refine existing macros** (provided by Keaton) — review, clean up, and
      either tighten as macros or, if a macro is insufficient, scope an addon.
  - [x] Smart pet attack + engage (hunter) → `macros/hunter/smart-pet-attack.md`.
        Primary = one-button Auto Shot + mouseover pet (target-splitting), tuned
        for a leveling Auto-Shot hunter. **Left to do:** test in-game (`!Auto
        Shot` toggle, mouseover split, optional `/startattack` melee fallback).
- [ ] **Hunter: feed-pet macro** — feed the pet the *best available* food in bags
      for its diet/level. Determine whether a pure macro can pick "best available"
      food, or whether this needs a small addon (likely needs bag scanning + diet
      logic → probably an addon fallback).
- [ ] **Rotation helper macros** — one-click / `castsequence` / `/cast` priority
      macros to smooth out rotations for specific specs (specs TBD).
- [ ] **Run `scripts/setup-luals.sh`** once an editor is chosen, to pull the WoW
      API EmmyLua annotations into `.luals/wow-api` (gitignored) for autocomplete
      + type checking. (API sourcing decided — see `docs/api/README.md`.)

## Setup (done)

- [x] Project structure (`macros/`, `addons/`, `docs/`, `config/`, `scripts/`).
- [x] README, conventions (`docs/conventions.md`), TODO.
- [x] AI-assist rules kept local-only in `CLAUDE.local.md` (gitignored), plus
      shared `.vscode/extensions.json`.
- [x] Privacy: gitignore local game paths (`config/paths.local.md`); commit a
      `config/paths.example.md` template.
- [x] Global rules: privacy + code-quality in `~/.claude/CLAUDE.md`.
- [x] API sourcing decided + documented (`docs/api/`); `.luarc.json` (Lua 5.1 +
      WoW API) and `scripts/setup-luals.sh` added.
- [x] Confirmed TBC Anniversary interface number: `20505` (patch 2.5.5).

## Backlog / ideas

- [ ] Helper script to sync macros/addons into the local WoW install (reads paths
      from gitignored `config/paths.lua`).
- [ ] Per-class macro packs.
