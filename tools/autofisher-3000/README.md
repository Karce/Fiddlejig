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
- **Detection** (`detect.rs`) — `CascadeDetector` runs a trained Haar cascade `.xml`
  to find bobber centers; `LureMatcher` template-matches the lure-buff icon. The
  `Detector` trait is the seam for a future learned backend (see Roadmap).
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

## Configuration

Defaults live in [`bot/src/config.rs`](bot/src/config.rs) (`Config`) — cast/lure keys,
the grace/settle/fishing/loot timings, the detection knobs (`min_neighbors`,
`stability_ms`, `flicker_ms`, `stability_radius`), and the lure icon/threshold. The
struct is `serde`-ready for a future `config.toml`; today the defaults are compiled in.

Notable tuning knobs:
- `min_neighbors` (cascade) — lower = more sensitive (fewer missed bites, more raw
  false positives, which the stability gate then filters).
- `stability_ms` / `flicker_ms` — how long a bobber must persist (and how much
  detection flicker is tolerated) before the bot commits to a reel.
- `post_catch_ms` — the loot pause after a catch.

## Tests & quality

```sh
cargo test                                  # pure state machine + detector smoke test
cargo test -- --ignored --nocapture         # run the cascade over the local corpus
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

The state machine has full unit coverage: grace → cast, settle, the stability gate
(reel-after-stable, flicker tolerance, transient-FP rejection), looting, and the lure
decision. The ignored test runs the default cascade over the local (gitignored)
training corpus.

## Roadmap

- **Pluggable detection backend** (the `Detector` trait). Haar cascades are fast
  (~ms) but brittle and need per-zone retraining. Swap for a learned detector — a
  small ONNX model (YOLO-n / RT-DETR) via the [`ort`](https://crates.io/crates/ort)
  crate, or a **local vision LLM** — gated on inference latency staying well under the
  fishing cadence (~0.5–2 s/cast).
- **Fold `training/` into the workspace.** It's the original screen-capture/train tool
  (Rust, `portal-screencast` + opencv `0.86`); that opencv crate predates system
  OpenCV 4.13, so it's currently a standalone, excluded crate. Migrate it to opencv
  `0.98` + `ashpd` and add it as a workspace member.
- **Resolution-independent lure matching** (scale the template / feature-match rather
  than a fixed-resolution PNG).
- **Restore-token persistence** so the portal dialog only appears on first run.

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
