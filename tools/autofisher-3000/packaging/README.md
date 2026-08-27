# Flatpak packaging

The Flatpak is the shippable artifact for Autofisher-3000. It bundles the whole
native stack — glibc, GStreamer, OpenCV and ONNX Runtime — so the app runs on any
Flatpak-capable system without installing anything on the host.

This replaces the old static-OpenCV build. That approach existed to avoid layering
`-devel` packages onto an immutable host, but it pinned the binary to the build
image's glibc and ran nowhere else.

## Build

Nothing needs to be installed system-wide; `flatpak-builder` itself is a Flatpak.

```sh
flatpak install -y flathub org.flatpak.Builder
cd tools/autofisher-3000
flatpak run org.flatpak.Builder --force-clean --user --install \
  --install-deps-from=flathub --repo=repo \
  build packaging/io.github.karce.Autofisher3000.yaml
```

Then:

```sh
flatpak run io.github.karce.Autofisher3000
```

A redistributable single file:

```sh
flatpak build-bundle repo autofisher-3000.flatpak io.github.karce.Autofisher3000
# install it elsewhere with:  flatpak install ./autofisher-3000.flatpak
```

## Regenerating `cargo-sources.json`

Flatpak builds run with no network, so every crate is vendored ahead of time.
Regenerate this file **whenever `Cargo.lock` changes**, or the build will fail
offline:

```sh
curl -O https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py
python3 flatpak-cargo-generator.py Cargo.lock -o packaging/cargo-sources.json
```

The devcontainer already has the script's dependencies (`aiohttp`, `PyYAML`,
`tomlkit`).

## Sandbox

The app holds essentially no permissions:

| arg | why |
|---|---|
| `--env=ORT_DYLIB_PATH=…` | not a permission; points `ort`'s `dlopen` at `/app/lib` |
| `--filesystem=xdg-pictures:create` | the only disk I/O — `--grab-frame` writes a PNG, `--check-lure` reads one |

Screen capture and synthetic input both go through XDG portals, which every
sandbox may talk to through the filtered D-Bus proxy, so they cost nothing. There
is no display socket, no PipeWire socket, and no network access. See the comments
in the manifest for what is deliberately absent and why.

## Known difference from the devcontainer build

`--debug` (the live detection window) does **not** work in the Flatpak. OpenCV is
built with `-DWITH_GTK=OFF`, so `highgui::named_window` throws. That keeps GTK3 and
a display socket out of the sandbox entirely. Use the devcontainer for debug work.
