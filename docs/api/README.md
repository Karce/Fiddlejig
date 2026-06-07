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

VS Code users can instead just install the **"WoW API"** extension
(`ketho.wow-api`) — it auto-loads the same annotations when you open a folder
containing a `.toc`.

## Patch / interface number

- Current TBC Anniversary patch: **2.5.5** → `.toc` `## Interface: 20505`.
- Re-confirm after each patch (in-game `select(4, GetBuildInfo())`, or
  [`Gethe/wow-ui-source`](https://github.com/Gethe/wow-ui-source)).

## The macro environment (why some things must be addons)

Macros run restricted: combat/protected actions only go through the secure
macro/`SecureActionButton` system, and there's a 255-character limit. Anything
needing protected actions in combat, persistent state, a UI, or longer logic must
be an addon — and even addons can't bypass combat protection on protected
functions. Keep this in mind for every macro-vs-addon call.
