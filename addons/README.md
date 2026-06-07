# Addons

Full addons live here, one folder per addon: `addons/<AddonName>/`, containing a
`<AddonName>.toc` and its Lua sources.

**Build an addon only as a fallback** — when a [macro](../macros/) genuinely can't
do the job (needs saved state, event handling, a UI/frame, or logic past the
255-character macro limit). When adding one, note in its README *why* a macro
wasn't enough.

## Installing

Copy the addon folder into your WoW install's
`_classic_/Interface/AddOns/` directory, then enable it on the character-select
addons screen. (A sync script that reads `config/paths.lua` is on the TODO.)

## .toc essentials (TBC Anniversary / 2.5.x)

```
## Interface: 20505
## Title: <AddonName>
## Notes: <what it does>
## Author: <you>
## Version: 0.1.0
## SavedVariables: <AddonNameDB>

<AddonName>.lua
```

> `20505` = patch 2.5.5 (TBC Anniversary). Re-confirm after each patch — see
> [`../docs/api/`](../docs/api/).
