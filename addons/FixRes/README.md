# FixRes

Auto-corrects broken startup resolution on some Linux compositors.

## Why not a macro?

Macros cannot fire automatically at login. FixRes needs
`PLAYER_ENTERING_WORLD` event handling to detect and correct a
resolution mismatch before any player interaction.

## Problem

On some Linux compositors via Proton, the WoW client starts with a
work-area-clipped resolution instead of the fullscreen resolution stored
in Config.wtf. The file is correct — the corruption is runtime-only in
the live GX state.

Manual fix: re-select the resolution in Graphics settings. FixRes
automates this at login.

## Commands

- `/fixres status` — show actual vs desired resolution
- `/fixres <W>x<H>` — set a resolution override (persists across sessions)
- `/fixres apply` — manually re-apply the desired resolution

On a healthy launch (actual matches desired), FixRes does nothing.
