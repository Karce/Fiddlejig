# Fiddlejig

Quality-of-life **macros**, **addons**, **WeakAuras**, and **tools** for
WoW Classic. Supports **TBC Anniversary** (2.5.6) and **Classic Era** (1.15.9).

## Macros first

Every feature starts as a macro. An addon gets built only when a macro can't
do the job: persistent state, event handling, UI, or logic past the 255-char
limit.

## What's here

| Category | Contents |
|---|---|
| **Addons** | [JojaAutoPetter](addons/JojaAutoPetter/) — auto-feed your hunter pet the best food in bags. [ShotClock](addons/ShotClock/) — hunter Auto Shot reload bar + aim window. |
| **Macros** | [Hunter](macros/hunter/), [Warrior](macros/warrior/) — rotation and utility macros. |
| **WeakAuras** | [Warrior](weakauras/warrior/) — Battle Shout / Rend reminders (importable `!WA:2!` strings). |
| **Tools** | [Autofisher-3000](tools/autofisher-3000/) — vision-based fishing bot (Rust + OpenCV, static-linked). [WA generator](tools/weakauras/) — builds deterministic WeakAura import strings from specs. |

See [`TODO.md`](TODO.md) for the backlog.

## Repository layout

```
Fiddlejig/
├── macros/<class>/          Documented macros (description + code fence)
├── addons/<Name>/           Full addons (.toc + Lua); dual-TOC for both clients
├── weakauras/<class>/       Importable WA strings, one per client flavor
├── tools/
│   ├── autofisher-3000/     Fishing bot (Rust); builds in distrobox
│   └── weakauras/           WA import-string generator (Lua)
├── docs/api/                WoW Classic TBC API reference
├── docs/conventions.md      Project conventions
├── types/                   Lua Language Server type stubs (WoW API)
├── config/paths.example.md  Local game-path template (fill in gitignored copy)
├── scripts/                 Dev helpers (LuaLS setup, etc.)
├── CLAUDE.md                AI-assist project rules
└── TODO.md                  Feature backlog
```

## Supported clients

Both clients load from the same addon sources via the dual-TOC pattern:
the `_Vanilla.toc` (Interface 11509) is preferred by Classic Era; the plain
`.toc` (Interface 20506) is loaded by TBC Anniversary.

## Getting started

1. Copy the config template and fill in your local game paths:
   ```sh
   cp config/paths.example.md config/paths.local.md
   ```
2. **Macros** — copy the code fence from any file under `macros/` into the
   in-game macro editor (`/macro`).
3. **Addons** — copy the addon folder into `Interface/AddOns/` and enable it
   on the character-select screen.
4. **WeakAuras** — paste the `!WA:2!` string for your client into `/wa` →
   Import. Requires the [WeakAuras](https://www.curseforge.com/wow/addons/weakauras-2) addon.

## CurseForge release pipeline

Each addon has its own GitHub Actions workflow (`.github/workflows/`).
On push to `main` touching an addon's directory, the workflow:

- Packages the addon into a zip with version `1.0.<run_number>` injected into all TOC files
- Resolves `## Interface:` numbers from both TOC flavors to CurseForge game-version IDs at run time
- Uploads the zip to CurseForge via the Upload API

**Required repo secret:** `CF_API_TOKEN` — a CurseForge API token with project upload permission.
Generate one at [CurseForge Author Console](https://authors.curseforge.com/) → API tokens.

A daily game-version watcher keeps `## Interface:` numbers current automatically:

- Polls the Blizzard build API at 17:00 UTC (after Tuesday maintenance); also `workflow_dispatch`
- Opens a PR with the TOC edits and merges it — the merge triggers both release workflows
- **Required repo secret:** `BUMP_PUSH_TOKEN` — fine-grained PAT scoped to this repo,
  **Contents: Read and write** + **Pull requests: Read and write** (pushes with the
  default `GITHUB_TOKEN` do not trigger downstream workflows)
- A release run may fail if CurseForge has not yet registered the new game version —
  re-run the release workflow manually once it does

## Contributing

See [`docs/conventions.md`](docs/conventions.md). This is a **public repo** —
never commit local paths or personal information.

## License

[GPL-3.0](LICENSE).
