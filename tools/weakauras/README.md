# tools/weakauras

Generates the `!WA:2!` import strings published under
[`weakauras/`](../../weakauras/), so the auras are reproducible from data tables
rather than opaque pasted blobs — and writes them straight into the docs.

[`generate.lua`](generate.lua) builds each aura's data table and runs WeakAuras'
real export pipeline — `LibSerialize:SerializeEx` → `LibDeflate:CompressDeflate` →
`LibDeflate:EncodeForPrint`, prefixed with `!WA:2!` — exactly as the in-game
**Export** does (see WeakAuras' `Transmission.lua`). It then **self-verifies** by
decoding each string back and asserting the trigger fields survived. Output is
**deterministic**: regenerating unchanged specs yields byte-identical strings (a
sorted `pairs` is installed before LibSerialize loads), so docs never churn.

Each aura is generated once per **client flavor** — TBC Anniversary (`toc 20505`)
and Classic Era (`toc 11508`). The flavors differ only in that version tag, which
is what keeps WeakAuras from warning "made for a different game version" on import.

## One-time setup

The script borrows the `LibSerialize`/`LibDeflate` copies inside your installed
WeakAuras, via **gitignored symlinks** (machine paths never reach the repo):

```sh
ln -s "<wow>/_anniversary_/Interface/AddOns/WeakAuras" tools/weakauras/.local/wa-anniversary
ln -s "<wow>/_classic_era_/Interface/AddOns/WeakAuras" tools/weakauras/.local/wa-era
```

(`mkdir -p tools/weakauras/.local` first; your `<wow>` root is in the gitignored
`config/paths.local.md`. `WA_LIBS=<path-to>/WeakAuras/Libs` overrides if set.)

## Running it

```sh
lua tools/weakauras/generate.lua          # print every aura × flavor
lua tools/weakauras/generate.lua --write  # patch the strings into the docs
```

`--write` finds each spec's doc (the `doc` field) and replaces the `!WA:2!` line
under the matching `### <flavor>` heading — the doc must already have that
section. It reports `updated`/`unchanged` per string; a run right after a commit
should be all-`unchanged`, which doubles as a regression check that code changes
didn't alter the aura data.

## Adding or changing an aura

Edit the `specs` list (and `iconAura`, if you need a field it doesn't expose yet)
in `generate.lua`, create the doc under `weakauras/` with a `### <flavor>` section
per flavor (any `!WA:2!` placeholder line inside), and run with `--write`. The
data tables mirror WeakAuras' schema for an `icon` region with `aura2`
(Buff/Debuff) triggers; field names come straight from the installed source
(`Types.lua` `data_stub`, `RegionTypes/Icon.lua`, `BuffTrigger2.lua`,
`SubRegionTypes/`, `Transmission.lua`).

Pinned to WeakAuras **5.21.7 / internalVersion 90** (verified identical on both
installs — re-check `Init.lua`/`WeakAuras.lua` via the symlinks when the addon
updates).
