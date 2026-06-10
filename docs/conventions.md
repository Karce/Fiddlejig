# Conventions

## Macro vs addon (the core decision)

Start with a macro. Reach for an addon only when a macro can't do it:

| Need | Macro? | Addon? |
|------|--------|--------|
| One-off cast / `castsequence` / modifier conditionals | ✅ | |
| Stay within 255 characters | ✅ | |
| Persistent state across sessions (SavedVariables) | | ✅ |
| React to events (combat log, bag updates, etc.) | | ✅ |
| Custom UI / frames / buttons | | ✅ |
| Scan bags and pick the "best" item by rules | | usually ✅ |

When you build an addon, record *why* a macro wasn't sufficient in its README.

## Macros

- One macro per file under `macros/<category>/<name>.md`.
- Keep the macro body ≤ 255 characters; note the character count if it's close.
- Use `#showtooltip` where helpful; prefer `[mod]`/`[@target]` conditionals over
  separate macros when it keeps you under the limit.

## Addons

- `addons/<AddonName>/<AddonName>.toc` + Lua sources.
- Namespace globals; prefer a single addon table (`local ADDON = ...`) passed via
  the `...` vararg to avoid polluting `_G`.
- Use SavedVariables (declared in the `.toc`) for persistence.

## WeakAuras

- One aura per file under `weakauras/<category>/<name>.md`: a description, a
  `### <client flavor>` section with the `!WA:2!` string per supported client
  (TBC Anniversary, Classic Era), and how it's wired.
- Don't hand-edit import strings. They're generated from data tables by
  `tools/weakauras/generate.lua` (which runs WeakAuras' own serialize/compress/encode
  and self-verifies); edit the spec there and run with `--write` to patch the docs.
- Prefer **Show On: No Aura(s) Found** for "maintain/reapply" reminders — it's the
  clearest and the most reliable mode on Classic clients.

## Lua style

- Clarity over cleverness; small single-purpose functions; comment the *why*,
  not the *what*; no dead code or leftover debug output.
- `local` everything you can; minimize global lookups in hot paths.

## Privacy

This is a public repo. No local paths, usernames, or personal info in commits.
Local paths go in the gitignored `config/paths.lua`.
