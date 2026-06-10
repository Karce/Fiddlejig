# WeakAuras

Importable [WeakAuras](https://www.curseforge.com/wow/addons/weakauras-2) for WoW
Classic — **TBC Anniversary (2.5.x)** and **Classic Era (1.15)** — grouped by
class or category in subfolders (`warrior/`, ...). Each aura ships one import
string per client flavor; they differ only in the client-version tag, so pick the
one matching your game (the other still imports, just with a version warning).

## How sharing a WeakAura works

A WeakAura is exported to a single **import string** that starts with `!WA:2!`.
The string is the aura's data table serialized, compressed, and text-encoded with
WeakAuras' own libraries. Anyone can recreate the aura by pasting that string into
the in-game importer — no files to copy.

**To import one:**

1. Type `/wa` to open WeakAuras.
2. Click **Import** (bottom-left of the window).
3. Paste the string and click **Import** on the preview.

**To update one you already imported:** importing the same aura again opens a
diff/update prompt — accept it to overwrite, or import as a copy.

Each aura here lives in its own Markdown file: a description, the `!WA:2!` string
in a code fence, an explanation of how it's wired, and tweaks you might want.

## How these are maintained

The strings aren't hand-written — they're generated from data tables by
[`tools/weakauras/generate.lua`](../tools/weakauras/generate.lua), which runs
WeakAuras' real serialize → compress → encode pipeline and self-verifies each
string by decoding it back. So the source of truth is the generator; the strings
in these docs are its output. See [`tools/weakauras/`](../tools/weakauras/) to
regenerate or add auras.

If you tweak an imported aura in-game and want to fold your change back in, use
**Export** in WeakAuras (right-click the aura → Export, or the Import/Export
menu), paste me the string, and I'll reconcile it with the generator.

## Why not macros?

Per the project's [macros-first rule](../macros/), these are auras, not macros:
they're passive, always-on *displays* (a reminder icon that watches a buff/debuff
and appears when you need to act). A macro is an action you trigger on a keypress —
it can't watch state and show you something. Different tool, different job, so
WeakAuras is the right home rather than an addon or a macro.
