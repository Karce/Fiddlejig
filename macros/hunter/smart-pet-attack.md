# Smart Pet Attack + Engage (Hunter)

One press does two jobs and lets you **split targets**:

- **You** keep shooting your **current target** (Auto Shot stays on it).
- **Your pet** attacks your **mouseover** — or your current target if you have no
  mouseover.

So you can hold fire on mob A while sending the pet at mob B, just by mousing
over B when you press.

```
#showtooltip Auto Shot
/petattack [@mouseover,harm,nodead][harm,nodead]
/cast [harm,nodead] !Auto Shot
```

How it works:

- `!Auto Shot` turns Auto Shot **on and keeps it on** — pressing again won't
  toggle it off.
- Auto Shot deliberately uses your **current target only** (no `@mouseover`), so
  mousing over another mob redirects the **pet** without pulling your fire off
  your target. That's the target-split.
- `[harm,nodead]` on the cast suppresses the error text when you have no target
  or a friendly one selected.

## Optional: melee fallback while leveling

Auto Shot can't fire inside ~8 yds. If a mob closes to melee, add `/startattack`
so you auto-swing your weapon instead of standing there:

```
#showtooltip Auto Shot
/petattack [@mouseover,harm,nodead][harm,nodead]
/cast [harm,nodead] !Auto Shot
/startattack [harm,nodead]
```

## Alternatives

**Pet only (manual control, no shooting).** Also works for **Warlock** — pure
`/petattack`:

```
/petattack [@mouseover,harm,nodead][harm,nodead]
```

**Pet control via modifiers** (recall / passive). Alt → call back, Ctrl →
passive, else smart attack:

```
/petfollow [mod:alt]
/petpassive [mod:ctrl]
/petattack [nomod,@mouseover,harm,nodead][nomod,harm,nodead]
```

## Notes

- **Keybind vs. in-macro modifiers:** binding to **Shift+1** makes `[mod:shift]`
  inside the macro always true (useless), and Shift+Alt+1 is a *different*
  keybind than Shift+1. The modifier variant only works if you bind to an
  unmodified key.
- Re-issuing `/petattack` at a target the pet is already on is a **no-op**, so
  spamming the engage button won't reset the pet's path.
- All variants are well under the **255-character** limit.
- Target API: WoW Classic TBC Anniversary (2.5.x). I can't run the game from
  here — verify the `!Auto Shot` toggle and the mouseover split feel right
  in-game (`/dump`, `/api` if anything's off).
