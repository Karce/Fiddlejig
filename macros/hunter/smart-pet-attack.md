# Smart Pet Attack + Engage (Hunter)

One press does two jobs and lets you **split targets**:

- **You** start shooting your **current target**.
- **Your pet** attacks your **mouseover** — or your current target if you have no
  mouseover.

So you can keep shooting mob A while sending the pet at mob B, just by mousing
over B when you press.

The engage line differs per client: on TBC, `/startattack` starts **Auto Shot** at
range; on Classic Era it only starts **melee** (and *cancels* Auto Shot), so the
Era version uses `/cast !Auto Shot` instead.

## TBC Anniversary (2.5.x)

```
#showtooltip Auto Shot
/petattack [@mouseover,harm,nodead][harm,nodead]
/startattack [harm,nodead]
```

- `/startattack` starts your attack on your **current target**: Auto Shot when in
  range, melee when a mob is in your face. Unlike `/cast Auto Shot`, it's **safe
  to spam** — pressing again does nothing if you're already attacking, so it
  never clips your shot timer, and out of range it simply does nothing (no error
  spam). This is why we use it instead of `!Auto Shot` here. (Confirmed in TBC:
  `/startattack` triggers Auto Shot when in range.)

## Classic Era (1.15)

```
#showtooltip Auto Shot
/petattack [@mouseover,harm,nodead][harm,nodead]
/cast [harm,nodead] !Auto Shot
```

- On Era, `/startattack` begins your **melee** swing only — and starting melee
  **cancels Auto Shot** (Era melee macros lean on exactly that to land the first
  hit faster while closing in). So the spam-safe ranged engage is
  `/cast !Auto Shot`: the `!` means "turn on, never toggle off", so mashing the
  key never stops or clips your shot timer.
- In melee range Auto Shot won't fire (8 yd minimum) and spamming this gives
  "Target is too close" — when a mob is on you, use the melee macro under
  Alternatives instead.

## How the split works (both clients)

- The engage line uses your **current target only** (no `@mouseover`), so mousing
  over another mob redirects the **pet** without pulling your fire off your
  target. That's the target-split.
- `[harm,nodead]` keeps every line from firing at a friendly or dead unit.
- `#showtooltip Auto Shot` gives the button a sensible icon plus Auto Shot's
  range coloring (reddens when out of range).

## Alternatives

**Era melee companion.** Queues Raptor Strike for the next swing and starts
melee — the `/startattack` here *deliberately* cancels Auto Shot so you swing the
moment the mob is in reach:

```
#showtooltip Raptor Strike
/cast Raptor Strike
/startattack
```

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

- **Why not `!Auto Shot` on TBC too?** It works, but `/startattack` is strictly
  better there: it also covers melee, and it's silent out of range where
  `/cast !Auto Shot` throws errors. Era just doesn't have that option for ranged.
- **Keybind vs. in-macro modifiers:** binding to **Shift+1** makes `[mod:shift]`
  inside the macro always true (useless), and Shift+Alt+1 is a *different*
  keybind than Shift+1. The modifier variant only works if you bind to an
  unmodified key.
- Re-issuing `/petattack` at a target the pet is already on is a **no-op**, so
  spamming the engage button won't reset the pet's path.
- All variants are well under the **255-character** limit.
- Target APIs: WoW Classic TBC Anniversary (2.5.x) and Classic Era (1.15).
