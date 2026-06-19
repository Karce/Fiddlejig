# Fiddlejig — project rules

Fiddlejig is a collection of tools (macros and addons) that aid the player and
automate tasks in **World of Warcraft Classic — The Burning Crusade Anniversary**
(WoW Classic 2.5.x). Written in **Lua** (plus the macro slash-command language).

This repo favors clarity and small, focused changes, and keeps personal info out of it
(it's public — see **Privacy**).

## Macros over addons

Prefer a **macro** for any feature that a macro can accomplish. Build a full
**addon only as a fallback** when a macro is genuinely insufficient (e.g. it needs
saved state, event handling, a frame/UI, or logic beyond the 255-character limit
and what the restricted macro environment allows). When proposing an addon,
briefly note *why* a macro won't do it.

## Privacy — this is a PUBLIC repo

- No local paths, usernames, machine names, or personal info in committed files.
- Local game install paths live in `config/paths.local.md` (**gitignored**) — a
  plain reference for the dev workflow (where to copy addons for testing). The
  committed template is `config/paths.example.md`.
- Scan staged diffs before every commit; stop and surface anything personal.

## Conventions

- Macros live under `macros/<class-or-category>/` as documented Markdown
  (description + the macro in a code fence + notes). Respect the 255-char limit.
- Addons live under `addons/<AddonName>/` with a `.toc` plus Lua sources.
- Target API: WoW Classic TBC Anniversary (2.5.x). See `docs/api/`.

## Building native code — build in a container, don't pollute the host

> **Applies to Fedora-based atomic/immutable desktops** — rpm-ostree / bootc systems like
> Fedora Silverblue & Kinoite and Universal Blue (Bluefin, Bazzite, Aurora). There it's
> load-bearing; on any other Linux the same build-in-a-container + static-link pattern is
> still good hygiene (clean host, reproducible builds).

On an atomic/immutable host the root filesystem is an image updated as a whole, and extra
packages are *layered* on top. Layering build/`-devel` packages is risky: one dep that
can't depsolve against the base image makes the **entire OS update silently roll back**.
(Concretely, layering `gstreamer1-plugins-base-devel` pulls `mesa-libgbm-devel`, which
won't resolve against the base image's `mesa-libgbm`, so `rpm-ostree`/`ujust update`
refuse to advance.) So:

- **Build in a container, not on the host.** When code needs a build dependency that
  isn't already present, install it inside a [distrobox](https://distrobox.it/)/toolbox
  (compilers, `-devel` headers, …) — never `rpm-ostree install` it onto the host.
- **Produce a binary that runs on the bare host.** Static-link the niche dependency (or
  otherwise avoid it at runtime) so the artifact needs only base-image libraries; confirm
  with `ldd` that the niche `.so` is gone.
- **Keep the host's layered-package set empty** (or minimal) so OS updates keep flowing.

Worked example — **Autofisher-3000** (`tools/autofisher-3000`), a Rust tool that links
**OpenCV statically**. `tools/autofisher-3000/devbox.sh` drives the flow:

```sh
cd tools/autofisher-3000
./devbox.sh box && ./devbox.sh deps && ./devbox.sh opencv   # one-time: container + static OpenCV
./devbox.sh build                                           # cargo build --release (static link)
./devbox.sh test                                            # or: ./devbox.sh in-box <cmd>
./target/release/autofisher-3000 --debug                    # runs on the host (ldd: no libopencv_*.so)
```

Reuse this pattern (`devbox.sh`-style wrapper + static link) for future native-dep tools.
