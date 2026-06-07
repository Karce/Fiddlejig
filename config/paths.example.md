# Local paths (example / template)

Copy this to `config/paths.local.md` and fill in your machine's paths.
`config/paths.local.md` is **gitignored** — it never gets committed.

```sh
cp config/paths.example.md config/paths.local.md
```

This file is a plain reference (not code). It tells the dev workflow where the
local WoW Classic install lives, so addons can be copied in for testing.

## Paths

- **WoW Classic root** (folder containing the `.exe` / the `_anniversary_` dir):
  `…/World of Warcraft/_anniversary_`
- **AddOns dir** (where addon folders get copied for testing):
  `…/World of Warcraft/_anniversary_/Interface/AddOns`
- **WTF dir** (account SavedVariables / macros, if needed):
  `…/World of Warcraft/_anniversary_/WTF`
- **Account name** (under `WTF/Account/<NAME>`): `…`
