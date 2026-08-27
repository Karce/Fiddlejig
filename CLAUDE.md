# Fiddlejig — project rules

Fiddlejig = collection of tools (macros + addons) that aid player and
automate tasks in **World of Warcraft Classic — The Burning Crusade Anniversary**
(WoW Classic 2.5.x). Written in **Lua** (plus macro slash-command language).

Repo favors clarity + small focused changes, keeps personal info out
(public — see **Privacy**).

## Macros over addons

Prefer **macro** for any feature macro can do. Build full
**addon only as fallback** when macro genuinely insufficient (e.g. needs
saved state, event handling, frame/UI, or logic beyond 255-char limit
and what restricted macro environment allows). When proposing addon,
note *why* macro won't do it.

## Privacy — this is a PUBLIC repo

- No local paths, usernames, machine names, personal info in committed files.
- Local game install paths live in `config/paths.local.md` (**gitignored**) —
  plain reference for dev workflow (where to copy addons for testing). Committed
  template = `config/paths.example.md`.
- Scan staged diffs before every commit; stop + surface anything personal.

## Conventions

- Macros live under `macros/<class-or-category>/` as documented Markdown
  (description + macro in code fence + notes). Respect 255-char limit.
- Addons live under `addons/<AddonName>/` with `.toc` plus Lua sources.
- Target API: WoW Classic TBC Anniversary (2.5.x). See `docs/api/`.

## Building native code — develop in a container, ship a Flatpak

Two separate jobs; don't conflate them.

**Developing** native-dependency code happens in the repo-wide
[devcontainer](https://containers.dev/) (`.devcontainer/`, Debian 13). It carries
compilers and `-devel` headers so none of that is ever installed on the host. This is
good hygiene anywhere, and load-bearing on image-based hosts (GNOME OS, Fedora
Silverblue/Kinoite, Universal Blue) where the root filesystem is an image and there may
be no package manager at all.

**Shipping** a desktop tool means a **Flatpak**, not a host binary. Do not try to make
the devcontainer emit something that runs on the bare host — that path pins the artifact
to the build image's glibc and it will run nowhere else. glibc has no forward
compatibility, and full static linking is not available to anything that needs GTK or
GStreamer (both are dynamic, and GStreamer `dlopen`s its plugins).

In a Flatpak, native dependencies become **manifest modules** built into `/app`. The
host stays clean because the app is sandboxed, not because the binary is static.

Worked example — **Autofisher-3000** (`tools/autofisher-3000`), Rust tool needing
OpenCV, GStreamer and ONNX Runtime.

Develop and test in the devcontainer:

```sh
# VS Code: "Reopen in Container" (or: devcontainer up --workspace-folder .)
cd tools/autofisher-3000
cargo build --release
cargo test
```

Build the shippable artifact on the host — `flatpak-builder` is itself a Flatpak, so
nothing is installed system-wide:

```sh
flatpak install -y flathub org.flatpak.Builder
cd tools/autofisher-3000
flatpak run org.flatpak.Builder --force-clean --user --install \
  --install-deps-from=flathub --repo=repo \
  build packaging/io.github.karce.Autofisher3000.yaml
```

See `tools/autofisher-3000/packaging/README.md`. Two rules that generalise to future
native-dep tools:

- **Flatpak builds have no network.** Vendor everything: Rust crates via
  `flatpak-cargo-generator.py` into `cargo-sources.json` (regenerate whenever
  `Cargo.lock` moves), and disable any dependency that fetches at configure time
  (OpenCV's `WITH_IPP`/`WITH_ADE` do).
- **Prefer portals to permissions.** Autofisher captures and injects input entirely
  through XDG portals, which every sandbox may use for free, so its `finish-args` are
  near-empty — no display socket, no PipeWire socket, no network.
