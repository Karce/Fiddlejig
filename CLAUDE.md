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

## Building native code — build in a container, don't pollute the host

> **Applies to Fedora-based atomic/immutable desktops** — rpm-ostree / bootc systems like
> Fedora Silverblue & Kinoite and Universal Blue (Bluefin, Bazzite, Aurora). There it's
> load-bearing; on any other Linux same build-in-a-container + static-link pattern still
> good hygiene (clean host, reproducible builds).

On atomic/immutable host, root filesystem = image updated as whole, extra
packages *layered* on top. Layering build/`-devel` packages risky: one dep that
can't depsolve against base image makes **entire OS update silently roll back**.
(Concretely, layering `gstreamer1-plugins-base-devel` pulls `mesa-libgbm-devel`, which
won't resolve against base image's `mesa-libgbm`, so `rpm-ostree`/`ujust update`
refuse to advance.) So:

- **Build in container, not host.** When code needs build dependency not
  already present, install inside a [devcontainer](https://containers.dev/)
  (compilers, `-devel` headers, …) — never `rpm-ostree install` onto host.
- **Produce binary that runs on bare host.** Static-link niche dependency (or
  otherwise avoid at runtime) so artifact needs only base-image libraries; confirm
  with `ldd` that niche `.so` gone.
- **Keep host's layered-package set empty** (or minimal) so OS updates keep flowing.

Worked example — **Autofisher-3000** (`tools/autofisher-3000`), Rust tool that links
**OpenCV statically**. The repo-wide devcontainer (`.devcontainer/`) carries the
toolchain and a pre-built static OpenCV:

```sh
# VS Code: "Reopen in Container" (or: devcontainer up --workspace-folder .)
cd tools/autofisher-3000
cargo build --release
cargo test
./target/release/autofisher-3000 --debug    # runs on the host (ldd: no libopencv_*.so)
```

Reuse this pattern (devcontainer with pre-built static deps) for future native-dep tools.