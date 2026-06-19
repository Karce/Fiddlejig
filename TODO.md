# Fiddlejig — TODO

Working document for features and tasks. Newest context at the top of each
section. **Policy:** prefer a macro; build an addon only when a macro can't do it.

## Now

- [ ] **Refine existing macros** (provided by Karce) — review, clean up, and
      either tighten as macros or, if a macro is insufficient, scope an addon.
  - [x] Smart pet attack + engage (hunter) → `macros/hunter/smart-pet-attack.md`.
        One-button engage + mouseover pet (target-splitting). Field-tested:
        `!Auto Shot` clipped when spammed/out of range, so switched to
        `/startattack` (spam-safe, fires Auto Shot in range, melees if closed on).
- [x] **Hunter: feed-pet** — feed the pet the *best available* food in bags for its
      diet/level → `addons/JojaAutoPetter/` (**Joja Mart's Auto-Petter**). **Needed an
      addon** (a macro can't scan bags, run diet/level logic, or feed a dynamically
      chosen bag slot through a secure button). Picks highest happiness tier →
      lowest-level food in it → smallest stack (conserves good food); reminds when
      happiness drops. Trigger by keybind or `/click JojaAutoPetterButton`.
- [ ] **Auto-Petter: auto-buy food** — at a vendor, auto-purchase compatible food the
      pet *loves* (best-fit, level-appropriate), with a toggle to turn it on/off. Scan
      the merchant (`GetMerchantNumItems`/`GetMerchantItemLink`), match against the diet
      + `Foods` table, buy via `BuyMerchantItem`. (Addon — needs merchant scan + state.)
- [ ] **Auto-Petter: auto-cook** — one button to cook the raw food in your bags when
      Cooking is known, in as few clicks as possible. Likely a secure cast + queue
      across the cookable raw mats you hold. (Addon — trade-skill scan + secure cast.)
- [ ] **Auto-Petter: low-food alert** — warn when you're running low on / out of
      suitable food for the pet's diet (count remaining feeds), so you can restock
      before it matters. Ties into auto-buy/auto-cook above.
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
