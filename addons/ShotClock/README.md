# ShotClock

A Fiddlejig addon: a minimal **Hunter Auto Shot timer** for Classic. In Classic,
every Auto Shot cycle ends in a ~0.5 second **aim window** — if you move or cast
during it, you delay your own shot. ShotClock draws that cycle as one bar:

- **Reload phase** — a green fill shrinks toward empty while the weapon reloads.
  You're free to move and weave casts.
- **Aim phase** — a red fill grows over the ~0.5s aim window. Stand still; if you
  move, the red fill visibly snaps back because your aim restarted.
- The number on the right is seconds until the shot fires.

The timer also models the cycle's edge cases: Aimed Shot restarting it, Feign
Death resetting it to the unhasted weapon speed (+0.15s) when broken by moving or
jumping, in-combat haste buffs rescaling it mid-reload, and the server's 0.5s
retry delay after a failed shot attempt (target too close, wrong facing).

The bar shows in combat or while shooting, and hides otherwise (always visible
while unlocked, so you can place it). Loads only on Hunters.

## Usage

Drag the bar where you want it, then `/sc lock`. Commands (`/shotclock` or `/sc`):

| Command | Effect |
|---|---|
| `/sc` | status + help |
| `/sc lock` / `/sc unlock` | lock (enable click-through) / unlock for dragging |
| `/sc width <px>` / `/sc height <px>` | bar size |
| `/sc scale <0.5–2>` | overall scale |
| `/sc text on\|off` | toggle the countdown number |
| `/sc reset` | restore defaults, including position |

## Why an addon (and not a macro)?

Per the project's macros-first rule: the timer needs continuous per-frame timing
(`OnUpdate`), combat-log and spellcast event handling, a drawn movable frame, and
a saved position — none of which exist in the restricted 255-character macro
environment.

## Attribution & license

The Auto Shot timer model is extracted from
**[WeaponSwingTimer](https://github.com/LeftHandedGlove/WeaponSwingTimerAddon)**
by **LeftHandedGlove**, as maintained in
**[WeaponSwingTimer-SixxFix](https://github.com/watchyoursixx/WeaponSwingTimer-SixxFix)**
by **WatchYourSixx** — huge thanks to both. Upstream ships no LICENSE file but
publicly states the code may be edited and modified. This extraction is a small
subset (the hunter Auto Shot engine and bar; melee/target/wand timers, castbar,
and config UI are not included) and is licensed with the rest of Fiddlejig under
GPL-3.0.

Deliberate behavior differences from upstream:

- Trueshot Aura **rank 4** (TBC, spell 27066) is included in the reset list —
  upstream only has ranks 1–3, so recasting r4 didn't reset its timer.
- A failed cast (`UNIT_SPELLCAST_FAILED`, e.g. target dies mid-Aimed-Shot) clears
  the casting state immediately instead of waiting for a 3s bailout.
- Spellcast events are player-filtered (`RegisterUnitEvent`), so other units'
  casts don't run the handlers at all.

## Install

Copy this `ShotClock/` folder into your WoW install's `Interface/AddOns/`
(`_anniversary_` for TBC Anniversary, `_classic_era_` for Classic Era), then
enable it on the character-select addons screen. The Era client loads
`ShotClock_Vanilla.toc` (Interface 11508); Anniversary falls back to the plain
`ShotClock.toc` (Interface 20505).
