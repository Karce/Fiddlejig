# WoW Classic TBC Anniversary (2.5.x) — Lua API reference

How we reference and type-check the WoW API while writing Fiddlejig.

## What's the *official* source of the API?

There is no single official API website. Blizzard's authoritative sources are:

1. **In-game API documentation** — Blizzard ships machine-readable API docs in the
   client via the `Blizzard_APIDocumentation` addon. Use `/api` in-game to browse
   it, and `/dump <expr>` to inspect values live. This is the ground truth for the
   exact version you're running.
2. **Blizzard's released UI source** — Blizzard publicly releases the WoW
   interface (FrameXML/AddOns Lua) each patch. The common mirror is
   [`Gethe/wow-ui-source`](https://github.com/Gethe/wow-ui-source) (Classic
   branches), which shows the real API surface and `.toc` Interface numbers.

Everything else is community-maintained on top of those:

- **Warcraft Wiki** — [World of Warcraft API](https://warcraft.wiki.gg/wiki/World_of_Warcraft_API)
  (successor to Wowpedia). De-facto reference; flags Classic-vs-Retail
  availability. Good for prose/examples.
- **Ketho's annotations** — see below; generated from Blizzard's docs/UI source.

> Note: the **Battle.net / `*.api.blizzard.com` Game Data APIs** are a *separate*
> thing — web REST APIs for armory/auction data, **not** the in-client addon Lua
> API we use here.

## EmmyLua / Lua Language Server setup (editor-agnostic)

We use the [Lua Language Server](https://github.com/LuaLS/lua-language-server)
(sumneko) with WoW API annotations from
[`Ketho/vscode-wow-api`](https://github.com/Ketho/vscode-wow-api). The
annotations are EmmyLua-style and are the most complete, current type defs for
the WoW API — they power autocomplete and type checking in **any** LuaLS-based
editor (VS Code, Neovim, Zed).

**Setup:**

```sh
scripts/setup-luals.sh   # clones the annotations into .luals/wow-api (gitignored)
```

`.luarc.json` (committed, at repo root) sets `runtime.version` to **Lua 5.1**
(WoW's Lua) and points `workspace.library` at `.luals/wow-api/Annotations`.

VS Code users can also install the **"WoW API"** extension (`ketho.wow-api`),
which bundles the same annotations. We still keep the explicit `.luarc.json` +
clone so the setup works in any editor and regardless of where the `.toc` lives.

> **Heads-up: these annotations are Retail-focused.** Classic-only APIs aren't in
> them (e.g. `GetPetHappiness`, removed from Retail in 4.1), and some globals we
> use are marked *deprecated* because Retail moved them under namespaces
> (`C_Container.*`, `C_Spell.*`) — in 2.5.x the globals are still correct. Fill
> genuine gaps with small local stubs in [`types/`](../../types/) (a `---@meta`
> file), which `.luarc.json` also loads as a library. Don't trust "undefined
> global" blindly on a Classic project — cross-check the in-game `/api` and the
> wiki's Classic flags.

## Patch / interface number

- Current TBC Anniversary patch: **2.5.6** → `.toc` `## Interface: 20506`.
- Re-confirm after each patch (in-game `select(4, GetBuildInfo())`, or
  [`Gethe/wow-ui-source`](https://github.com/Gethe/wow-ui-source)).

## The macro environment (why some things must be addons)

Macros run restricted: combat/protected actions only go through the secure
macro/`SecureActionButton` system, and there's a 255-character limit. Anything
needing protected actions in combat, persistent state, a UI, or longer logic must
be an addon — and even addons can't bypass combat protection on protected
functions. Keep this in mind for every macro-vs-addon call.

## Secure frames & key bindings (the parts annotations DON'T cover)

The LuaLS annotations give you function *signatures*, not the *semantics* of the
secure system. That gap cost us a lot of trial-and-error on the feed-pet addon, so
the hard-won rules are written down here. Authoritative sources:

- Warcraft Wiki — [SecureActionButtonTemplate](https://warcraft.wiki.gg/wiki/SecureActionButtonTemplate),
  [SecureActionButton attributes](https://warcraft.wiki.gg/wiki/SecureActionButton),
  [Creating Custom Key Bindings (Bindings.xml)](https://warcraft.wiki.gg/wiki/Creating_custom_key_bindings),
  [API_SetOverrideBindingClick](https://warcraft.wiki.gg/wiki/API_SetOverrideBindingClick).
- Blizzard UI source — `FrameXML/SecureTemplates.lua` in
  [`Gethe/wow-ui-source`](https://github.com/Gethe/wow-ui-source) (the actual
  attribute-resolution code).
- **Read a working installed addon.** The fastest ground truth for "does X work in
  *this* client" is another addon doing X. RXPGuides' keybindable item buttons
  (`ActiveItemFrame.lua` + its `Bindings.xml`) were our reference.

**Performing a protected action on a chosen item (e.g. cast Feed Pet on a bag slot):**

- Use a `CreateFrame("Button", name, parent, "SecureActionButtonTemplate")` and set
  its action via attributes **out of combat** (`InCombatLockdown()` to gate). Two
  equivalent forms:
  - macro: `SetAttribute("type1","macro")` + `SetAttribute("macrotext", "/cast Feed Pet\n/use <bag> <slot>")`
  - spell-on-item: `SetAttribute("type1","spell")` + `spell` + `target-bag`/`target-slot`
- The `1` suffix (`type1`) is the **LeftButton** action; resolution falls back from
  `type1` → `type`. Match the mouse button your trigger uses.

**Triggering it from a keypress — the part that bit us:**

- A plain Lua `someButton:Click()` from a slash handler or a `<Binding>` body does
  **NOT** fire the secure action. The trigger must be a real *secure* click:
  - a **`CLICK <ButtonName>:LeftButton`** binding in `Bindings.xml` (canonical), or
  - a `/click <ButtonName>` macro on the action bar, or
  - `SetOverrideBindingClick(owner, isPriority, key, "<ButtonName>")`.
- **`Bindings.xml` is auto-loaded by WoW from the addon folder — never list it in
  the `.toc`.** Listing it makes WoW parse it a *second* time under the UI-widget
  schema → "Unrecognized XML: Binding" errors. (Every other addon omits it.)
- Section header + label globals: the binding's `category="BINDING_HEADER_Foo"`
  points at a `_G.BINDING_HEADER_Foo = "Display Name"`, and
  `_G["BINDING_NAME_CLICK ButtonName:LeftButton"] = "Label"` names the row.
- Turn on **`/console scriptErrors 1`** while developing — WoW hides Lua errors by
  default. A self-serve `/<addon> debug` dump (diet, bag scan, the armed
  attributes, the bound key) is worth its weight for diagnosing secure issues.
