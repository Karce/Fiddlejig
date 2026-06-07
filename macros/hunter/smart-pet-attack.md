# Smart Pet Attack + Engage (Hunter)

One press does two jobs and lets you **split targets**:

- **You** attack your **current target** — `/startattack` fires Auto Shot at
  range (and melee-swings if a mob reaches you).
- **Your pet** attacks your **mouseover** — or your current target if you have no
  mouseover.

So you can keep shooting mob A while sending the pet at mob B, just by mousing
over B when you press.

```
#showtooltip Auto Shot
/petattack [@mouseover,harm,nodead][harm,nodead]
/startattack [harm,nodead]
```

How it works:

- `/startattack` starts your attack on your **current target**: Auto Shot when in
  range, melee when a mob is in your face. Unlike `/cast Auto Shot`, it's **safe
  to spam** — pressing again does nothing if you're already attacking, so it
  never clips your shot timer, and out of range it simply does nothing (no error
  spam). This is why we use it instead of `!Auto Shot`.
- `/startattack` uses your **current target only** (no `@mouseover`), so mousing
  over another mob redirects the **pet** without pulling your fire off your
  target. That's the target-split.
- `[harm,nodead]` keeps both lines from firing at a friendly or dead unit.
- `#showtooltip Auto Shot` gives the button a sensible icon plus Auto Shot's
  range coloring (reddens when out of range).

## Alternatives

**Pet only (manual control, no attacking).** Also works for **Warlock** — pure
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

- **Why not `/cast !Auto Shot`?** Auto Shot is auto-repeat — it only needs
  starting *once*. Spamming a `/cast Auto Shot` line re-pokes it every press,
  clipping the shot timer, and throws errors out of range. `/startattack` avoids
  both. (Confirmed in TBC: `/startattack` triggers Auto Shot when in range.)
- **Keybind vs. in-macro modifiers:** binding to **Shift+1** makes `[mod:shift]`
  inside the macro always true (useless), and Shift+Alt+1 is a *different*
  keybind than Shift+1. The modifier variant only works if you bind to an
  unmodified key.
- Re-issuing `/petattack` at a target the pet is already on is a **no-op**, so
  spamming the engage button won't reset the pet's path.
- All variants are well under the **255-character** limit.
- Target API: WoW Classic TBC Anniversary (2.5.x).
