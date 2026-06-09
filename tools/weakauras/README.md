# tools/weakauras

Generates the `!WA:2!` import strings published under
[`weakauras/`](../../weakauras/), so the auras are reproducible from data tables
rather than opaque pasted blobs.

[`generate.lua`](generate.lua) builds each aura's data table and runs WeakAuras'
real export pipeline — `LibSerialize:SerializeEx` → `LibDeflate:CompressDeflate` →
`LibDeflate:EncodeForPrint`, prefixed with `!WA:2!` — exactly as the in-game
**Export** does (see WeakAuras' `Transmission.lua`). It then **self-verifies** by
decoding each string back and asserting the trigger fields survived.

## Running it

It borrows the `LibSerialize`/`LibDeflate` copies that ship inside your installed
WeakAuras, via the `WA_LIBS` environment variable (a local, machine-specific path —
kept out of the repo). Point it at your `WeakAuras/Libs` directory:

```sh
WA_LIBS="/path/to/Interface/AddOns/WeakAuras/Libs" lua tools/weakauras/generate.lua
```

It prints each aura's id and its import string. Copy the strings into the matching
docs under `weakauras/`.

Any standalone Lua 5.1+ works (a `unpack`/`table.unpack` shim inside the script
covers both). Your WoW path lives in the gitignored `config/paths.local.md`.

## Adding or changing an aura

Edit the `specs` list (and `iconAura`, if you need a field it doesn't expose yet)
in `generate.lua`, re-run, and paste the new strings into `weakauras/`. The data
tables mirror WeakAuras' schema for an `icon` region with an `aura2` (Buff/Debuff)
trigger; the field names come straight from the installed source
(`Types.lua` `data_stub`, `RegionTypes/Icon.lua`, `BuffTrigger2.lua`,
`Transmission.lua`). Pinned to WeakAuras 5.21.7 / Interface 20505.
