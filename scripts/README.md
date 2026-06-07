# Scripts

Helper scripts for development and installation.

Anything that touches your local machine (WoW install path, AddOns dir) must read
those values from the gitignored `config/paths.local.md` — never hardcode local
paths (this is a public repo).

## `setup-luals.sh`

Fetches the WoW API EmmyLua/LuaLS annotations into `.luals/wow-api` (gitignored)
so any LuaLS-based editor gets WoW API autocomplete + type checking. Run once:

```sh
scripts/setup-luals.sh
```

> **Planned:** a sync script that copies addons from `addons/` into the local
> `Interface/AddOns` directory using `config/paths.lua`. See
> [`../TODO.md`](../TODO.md).
