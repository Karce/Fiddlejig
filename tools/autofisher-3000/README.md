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
  selected by config: `CascadeDetector` (OpenCV Haar cascade, default) and
  `NnDetector` (a trained YOLO11-n ONNX run in pure Rust via [`tract`](https://crates.io/crates/tract)).
  `LureMatcher` template-matches the lure-buff icon. See **Detector backends** below.
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

## Build (Fedora)

```sh
sudo dnf install opencv-devel gstreamer1-devel gstreamer1-plugins-base-devel clang
cd tools/autofisher-3000
cargo build --release
```

Requirements: a native **Wayland GNOME** session (its portal backend
`gnome-remote-desktop` provides both ScreenCast and RemoteDesktop), **PipeWire** + the
`pipewiresrc` GStreamer element, **OpenCV** built with GStreamer (Fedora's is), and
`clang`/`libclang` for the `opencv` crate's bindgen. `Cargo.lock` is committed.

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

The bot template-matches your lure-buff icon (`icons/lure_2560x1440.png`) anywhere in
the frame, ~every 2s. When the buff is missing it presses the lure key, waits
`lure_cast_secs`, then casts. The template is **resolution/UI-specific** — for a
different resolution or lure, capture a fresh one:

1. Apply the lure in-game, then `--grab-frame frame.png`.
2. Crop **just the icon art** (no border/background) to `icons/lure_<WxH>.png`.
3. Point `Config.lure_icon` at it and verify with `--check-lure frame.png` (aim for a
   score well above the `lure_threshold`).

## Detector backends

Two detectors sit behind the `Detector` trait, picked by `Config::backend`:

- **`cascade`** (default) — the OpenCV Haar cascade. Fast and cheap, but
  scenery-sensitive: it needs per-zone retraining and misses bites in zones it wasn't
  trained on.
- **`nn`** — a YOLO11-n detector trained on the same splash labels, exported to ONNX
  and run in pure Rust via `tract` (no native runtime). Far more robust across zones.

Measured over the local corpus (447 images; `cargo test --release compare_cascade_vs_nn
-- --ignored --nocapture`):

| backend | recall (splash) | false-pos | inference (CPU, 960²) |
|---|---|---|---|
| cascade | 79.4% | 0.3% | ~42 ms |
| nn (YOLO11-n) | **98.5%** | 1.6% | ~327 ms |

The NN is the clear accuracy winner (and held-out val mAP@50 was 0.995). It is **not
yet the default** because at 960² on CPU it runs ~327 ms/inference (~3 fps) — heavier
than the cascade, which works against the low-CPU goal. To make it the default,
either:
- **shrink the input** — retrain on **ROI crops** at a small `imgsz` (so the bobber
  stays large) and set a `roi` + smaller `nn_input_size`; or
- **swap the runtime to [`ort`](https://crates.io/crates/ort)** (ONNX Runtime) for a
  multi-threaded ~3–5× speedup at full 960² — the `Detector` trait makes this a
  drop-in for `NnDetector`.

Try the NN today with `backend = nn` (accepting the higher CPU). Training/export is
documented in [`training/nn/README.md`](training/nn/README.md).

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

```sh
cargo test                                  # pure state machine + both detector smoke tests
cargo test --release compare_cascade_vs_nn -- --ignored --nocapture   # backend comparison
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

The state machine has full unit coverage: grace → cast, settle, the stability gate
(reel-after-stable, flicker tolerance, transient-FP rejection), looting, and the lure
decision. Smoke tests load each detector (the cascade `.xml` and the `tract` ONNX) and
run them on a blank frame. The ignored tests run each backend over the local
(gitignored) corpus and print recall / false-positive / timing per backend.

## Roadmap

- **Learned detection backend — done, pending perf.** A YOLO11-n ONNX (`NnDetector`,
  `tract`) is implemented and beats the cascade on recall (98.5% vs 79.4%; see
  **Detector backends**). Remaining work to make it the default: cut its ~327 ms CPU
  inference via an ROI-crop model at small `imgsz`, or swap `tract` → `ort` for a
  multi-threaded speedup. (A local vision LLM stays out of scope — far too slow for
  the per-frame splash window.)
- **Fold `training/` into the workspace.** It's the original screen-capture/train tool
  (Rust, `portal-screencast` + opencv `0.86`); that opencv crate predates system
  OpenCV 4.13, so it's currently a standalone, excluded crate. Migrate it to opencv
  `0.98` + `ashpd` and add it as a workspace member.
- **Resolution-independent lure matching** (scale the template / feature-match rather
  than a fixed-resolution PNG).
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
