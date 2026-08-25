# Autofisher-3000

A World of Warcraft fishing bot for **GNOME / Wayland**, written in Rust. It watches
the game through the desktop **ScreenCast portal**, finds the bobber with an **OpenCV
Haar cascade**, and reels it in through the **RemoteDesktop portal** — so capture and
input share one coordinate space (no calibration) and it runs natively on Wayland.

> ⚠️ Game automation violates Blizzard's Terms of Service and can get an account
> actioned. This is a personal tool — use it at your own risk.

## How it works — functional core, imperative shell

```
ashpd: one ScreenCast + RemoteDesktop portal session
   │ PipeWire fd+node                         ▲ NotifyPointerMotionAbsolute / Button / Keycode
   ▼                                          │
GStreamer pipewiresrc → OpenCV VideoCapture → Frame
   → CascadeDetector (bobber) + LureMatcher (lure buff)
   → step()  [pure, typed state machine]  → [Action] → portal input
```

- **Capture** (`portal.rs`, `capture.rs`) — [`ashpd`](https://crates.io/crates/ashpd)
  opens one linked ScreenCast+RemoteDesktop session; OpenCV's `VideoCapture`
  (GStreamer / `pipewiresrc`) pulls frames as `Mat`s on a dedicated thread.
- **Detection** (`detect.rs`, `nn.rs`) — two backends behind the `Detector` trait,
  selected by config: `NnDetector` (a trained YOLO11-n ONNX via ONNX Runtime
  [`ort`](https://crates.io/crates/ort), **default**) and `CascadeDetector` (OpenCV Haar
  cascade). `LureMatcher` template-matches the lure-buff icon. See **Detector backends**.
- **Logic** (`state.rs`) — a **pure** `step(state, ctx) -> (state, [Action])`. No I/O,
  no clock reads, no sleeps; timing is encoded as deadlines in the state. Invalid
  states are unrepresentable, so the whole loop is exhaustively unit-tested with zero
  mocks.
- **Input** (`portal.rs`) — the RemoteDesktop portal injects pointer motion/buttons +
  keycodes in the **same coordinates** as the captured frame.

Because input goes through the portal in stream coordinates, **frame pixel (x,y) ==
click (x,y)** — there is no capture→screen calibration step.

### Why a Rust app (not a WoW macro/addon)
Real-time framebuffer capture, computer-vision detection, and synthetic OS input are
all outside what an in-game macro or addon can reach. This is a desktop automation
tool, not an addon.

## The state machine

```
Initializing → Casting → (ApplyingLure →) Settling → Searching → Looting → Casting → …
```

- **Initializing** — startup grace so you can focus the game.
- **Casting** — decide lure-vs-cast; emit the cast keypress(es) + park the cursor.
- **ApplyingLure** — only if the lure buff is missing: press the lure key, wait for it.
- **Settling** — for `settle_secs` after a cast, ignore the water (cast animation /
  idle bobber) so it can't false-trigger or reel early.
- **Searching** — confirm a bobber is **stable** (`stability_ms`, tolerating
  `flicker_ms` of detection drop-out, within `stability_radius` px) before reeling;
  recast if nothing bites within `fishing_duration_secs`.
- **Looting** — pause `post_catch_ms` after a reel so loot/animation resolves before
  recasting (so a lure re-apply keypress isn't swallowed by the catch).

## Build (devcontainer, statically linked OpenCV)

OpenCV is linked **statically**, so the release binary runs on a stock GNOME/Wayland host
with **no system OpenCV — and no `-devel` packages — installed**. The build runs inside a
VS Code [devcontainer](https://containers.dev/) (`.devcontainer/` at the repo root) that
carries the toolchain and a pre-built static OpenCV; only the base-image
GStreamer/GTK/glib libraries stay dynamic (they ship with the desktop).

```sh
# VS Code: "Reopen in Container" (or: devcontainer up --workspace-folder /path/to/Fiddlejig)
cd tools/autofisher-3000
cargo build --release
```

The binary lands at `target/release/autofisher-3000` and runs **directly on the host** —
confirm with `ldd target/release/autofisher-3000` (no `libopencv_*.so` should appear).
The first container build takes ~15–40 minutes (OpenCV compile); subsequent builds use
the cached image.

**Why static / why a container:** the `opencv` crate links OpenCV's shared libraries by
default, which would force `opencv` plus a chain of `-devel` packages to be layered onto an
image-based host like Bluefin — and that layering can block OS image updates. The
devcontainer pre-builds a static OpenCV into the container image, keeping the host free of
layered packages. A *fully* static (musl) binary isn't possible here because capture relies
on GStreamer dlopening `libgstpipewire.so` at runtime, so the GStreamer/GTK stack stays
dynamic (it's part of the base GNOME image).

Runtime requirements (host): a native **Wayland GNOME** session (its portal backend
`gnome-remote-desktop` provides ScreenCast + RemoteDesktop), **PipeWire** with the
`pipewiresrc` GStreamer element, and the base GTK/GStreamer libraries — all standard on a
Fedora/Bluefin GNOME desktop. No OpenCV package needed. `Cargo.lock` is committed. The NN
backend bundles ONNX Runtime via [`ort`](https://crates.io/crates/ort) (its
`download-binaries` feature fetches a prebuilt ORT at build time, so the first build needs
network).

## Run

```sh
./target/release/autofisher-3000 --debug
```

- **First run:** GNOME shows a *"share your screen + allow remote control"* dialog →
  pick your **game monitor** (or the **WoW window**) and allow. Keep the screen
  **unlocked** while it runs — GNOME inhibits the portal on the lock screen.
- After a 10s grace it casts, watches the water, reels a stable bobber, recasts.
- Press **`q`** in the debug window (or Ctrl-C) to stop.

CLI flags:

| flag | what it does |
|---|---|
| `--debug` | live window with detection boxes drawn |
| `--grab-frame <PATH>` | save one frame (use it to crop a lure icon) |
| `--check-lure <PATH>` | score the lure template against a saved frame, offline |
| `--model <PATH>` | use a different cascade `.xml` |

## In-game setup

- **Borderless windowed** (not exclusive fullscreen) so the window is capturable.
- **Fishing on action-bar slot `1`** (`cast_keys`), and turn **Auto Loot ON**.
- For auto-lure, a one-button **lure macro on slot `3`** (`lure_key`):
  ```
  #showtooltip
  /use Aquadynamic Fish Attractor
  /use 16
  ```
  (swap in your lure; `/use 16` applies it to your equipped pole).

## Auto-lure

The bot template-matches your lure-buff icon (`icons/lure_2560x1440.png`) in the
top-right buff area, once per cast. When the buff is missing it presses the lure key,
waits `lure_cast_secs`, then casts. The match is **multi-scale**: the one canonical
template (captured at 2560x1440) is pre-downscaled across a scale band, so it works at
**any render resolution** without a per-resolution PNG — 1080p, 1440p, etc. (resolutions
*above* 1440p aren't covered yet; see the Roadmap).

You only need a fresh template if you switch to a **different lure** (a different buff
icon):

1. Apply the lure in-game, then `--grab-frame frame.png`.
2. Crop **just the icon art** (no border/background) from your highest available
   resolution to `icons/lure_<WxH>.png`.
3. Point `Config.lure_icon` at it and verify with `--check-lure frame.png` (aim for a
   score well above the `lure_threshold`).

## Detector backends

Two detectors sit behind the `Detector` trait, picked by `Config::backend`:

- **`nn`** (default) — a YOLO11-n detector trained on the splash labels, exported to
  ONNX and run via ONNX Runtime (`ort`, multi-threaded). Robust across zones.
- **`cascade`** — the OpenCV Haar cascade. Scenery-sensitive: it needs per-zone
  retraining and misses bites in zones it wasn't trained on. Kept as a fallback.

Measured over the local corpus (447 images; `cargo test --release compare_cascade_vs_nn
-- --ignored --nocapture`):

| backend | recall (splash) | false-pos | inference (CPU, 960²) |
|---|---|---|---|
| cascade | 79.4% | 0.3% | ~42 ms |
| nn (YOLO11-n, ort) | **98.5%** | 1.6% | **~25 ms** |
| **nn + top-half ROI** (default) | 97.1% | 1.6% | ~25 ms |

The NN wins on **both** recall (98.5% vs 79.4%; held-out val mAP@50 = 0.995) **and**
speed (ORT is multi-threaded — ~13× faster than a single-threaded pure-Rust runtime, and
faster than the cascade), so it is the **default**.

**On the ROI** (`Config::roi`, normalized `x,y,w,h`): the default crops to the **top half**
(`(0, 0, 1, 0.5)`). Full width keeps the bobber at its trained scale (so recall barely
moves — 97.1% vs 98.5%), while dropping the bottom half — where the bobber never lands but
the action bars / character do — trims in-game false positives. A *tighter* crop (e.g.
upper-middle) shrinks the bobber ~2× and loses far casts, so it hurts recall (~91%); avoid
it unless you also retrain on ROI crops at a small `imgsz` (which would additionally buy
speed). Set `roi = None` for full-frame.

## Configuration

Defaults live in [`bot/src/config.rs`](bot/src/config.rs) (`Config`) — the detector
`backend` + its NN knobs, `target_fps`, an optional detection `roi`, cast/lure keys,
the grace/settle/fishing/loot timings, the cascade/stability knobs, and the lure
icon/threshold. The struct is `serde`-ready for a future `config.toml`; today the
defaults are compiled in.

Notable tuning knobs:
- `backend` — `cascade` or `nn` (see **Detector backends**).
- `target_fps` — capture rate cap (GStreamer `videorate`); cutting ~60→10 fps slashes
  the per-frame GPU readback + detection cost. Keep it above the splash duration.
- `nn_input_size` / `nn_conf_threshold` / `nn_iou_threshold` / `roi` — NN input edge
  (must match the export `imgsz`), confidence + NMS gates, and an optional normalized
  crop region (click coords are always mapped back to the full frame).
- `min_neighbors` (cascade) — lower = more sensitive (fewer missed bites, more raw
  false positives, which the stability gate then filters).
- `stability_ms` / `flicker_ms` — how long a bobber must persist (and how much
  detection flicker is tolerated) before the bot commits to a reel.
- `post_catch_ms` — the loot pause after a catch.

## Tests & quality

Run these inside the devcontainer (they need the static OpenCV):

```sh
cargo test                                  # pure state machine + both detector smoke tests
cargo test --release compare_cascade_vs_nn -- --ignored --nocapture
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

The state machine has full unit coverage: grace → cast, settle, the stability gate
(reel-after-stable, flicker tolerance, transient-FP rejection), looting, and the lure
decision. Smoke tests load each detector (the cascade `.xml` and the ORT ONNX) and
run them on a blank frame. The ignored tests run each backend over the local
(gitignored) corpus and print recall / false-positive / timing per backend.

## Roadmap

- **Auto-stop + log out when bags are full.** Right now the bot keeps fishing (and the
  character stays logged in) long after the bags fill — burning an online session for
  nothing. Goal: detect a full inventory, then **log the character out** (or quit the
  game), and stop the bot. A WoW *macro* can't do this (it needs `BAG_UPDATE` event
  handling, bag-slot queries, and a conditional logout), so it warrants a small
  **addon**: on `BAG_UPDATE`, sum free slots across bags 0–4 and, when zero, call
  `Logout()` (to char-select) or `Quit()` (exit the client). The bot then stops on its
  own when the capture stream ends — make that path exit *cleanly* (treat stream-end as
  a normal stop, not the current "capture pipeline did not open" error). Open questions:
  the exact TBC 2.5.x bag API (likely `C_Container.GetContainerNumFreeSlots`, possibly
  the legacy global `GetContainerNumFreeSlots`) and whether `Quit()`/`Logout()` are
  callable un-prompted out of combat. Alternative (no addon): OCR/template-match the red
  *"Inventory is full"* loot error and have the bot press a `/logout` keybind — less
  reliable than the game's own bag count. Consider a configurable safety cap (e.g. also
  stop after N hours) for unattended runs.
- **Learned detection backend — done.** A YOLO11-n ONNX (`NnDetector`, ONNX Runtime via
  `ort`) is the default: it beats the cascade on recall (98.5% vs 79.4%) **and** speed
  (~25 ms vs ~42 ms; see **Detector backends**). Optional follow-up: an ROI-crop model at
  a small `imgsz` to gain headroom + trim false positives. (A local vision LLM stays out
  of scope — far too slow for the per-frame splash window.)
- **Fold `training/` into the workspace.** It's the original screen-capture/train tool
  (Rust, `portal-screencast` + opencv `0.86`); that opencv crate predates system
  OpenCV 4.13, so it's currently a standalone, excluded crate. Migrate it to opencv
  `0.98` + `ashpd` and add it as a workspace member.
- **Resolution-independent lure matching — done.** The lure matcher (`LureMatcher`)
  pre-downscales one canonical template across a scale band and takes the best match, so
  a single 2560x1440 PNG works at any resolution at/below 1440p (no per-resolution PNGs).
  Follow-up: resolutions *above* the canonical (e.g. 4K) need a larger template — the
  buff icon is bigger than canonical there, and upscaling a ~34px template blurs it. Fix
  by capturing a higher-res canonical (e.g. `lure_3840x2160.png`) and selecting it (or
  the band) by frame resolution; `Config::lure_icon` already lets you point at one.
- **Skipping the portal dialog isn't possible** while input goes through the
  RemoteDesktop portal — GNOME refuses to persist input-injecting sessions
  (`org.freedesktop.portal.Error.InvalidArgument: Remote desktop sessions cannot
  persist`). It would require moving input to uinput/ydotool with a ScreenCast-only
  (persistable) capture session, at the cost of a coordinate-calibration step.

## Layout

```
tools/autofisher-3000/
├── Cargo.toml          # workspace (member: bot; training excluded — see Roadmap)
├── bot/
│   ├── Cargo.toml
│   ├── src/            # main, portal, capture, detect, state, config, frame
│   ├── models/         # trained Haar cascades (.xml)
│   └── icons/          # lure-buff templates
└── training/           # legacy capture+train tool (standalone crate; opencv 0.86)
```

## Provenance & license

The fishing concept and the cascade models descend from the MIT-licensed *OpenCV
Object Detection in Games* tutorial by Ben Johnson ("Learn Code By Gaming",
<https://github.com/learncodebygaming/opencv_tutorials>). The Rust implementation, the
portal capture/input, and the typed state machine are original. MIT.
