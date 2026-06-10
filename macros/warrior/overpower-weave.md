# Overpower Weave (Warrior)

Folds **Overpower** into the Heroic Strike button so it fires the moment it's
available, with no extra bar space and no extra thought. Overpower can't be
dodged, parried, or blocked and costs only 5 rage — on a leveling (especially
hardcore) warrior it's free damage you don't want to leave on the table.

```
#showtooltip Heroic Strike
/cast Overpower
/cast Heroic Strike
/startattack
```

Spam it as your main button: every press tries Overpower, queues Heroic Strike,
and keeps your auto-attack going.

How it works:

- `/cast Overpower` only succeeds inside the proc window (target dodged in the
  last ~5s, Battle Stance, 5+ rage, its 5s cooldown up). With a proc, one press
  fires Overpower *and* queues Heroic Strike.
- **This only works because Heroic Strike is off the GCD** (an on-next-swing
  attack). On these clients a failed `/cast` of any spell that *would* trigger the
  global cooldown blocks every later `/cast` in the macro
  ([macro rules](https://wowpedia.fandom.com/wiki/Making_a_macro)) — so a
  "priority" macro like `/cast Overpower` then `/cast Rend` casts Rend **never**.
  The fall-through priority macros in old guides are a 1.12-original-client
  behavior; Classic Era/Anniversary run the modern macro engine. Don't pair
  Overpower with another GCD ability.
- `/startattack` is spam-safe (it only ever *starts* your swing, unlike the
  default Attack action, which toggles it off), and with no target it acquires the
  nearest enemy in front of you — so this button also replaces a dedicated
  attack/engage key.

## Seeing the proc

The macro solves *pressing* Overpower; you still need to *notice* it's available.
Either keep Overpower itself on a visible action-bar slot (it un-greys when the
proc is live), or — nicer — a small WeakAura that lights up when Overpower becomes
usable, in the style of the [warrior auras](../../weakauras/warrior/) already in
this repo.

## Alternatives

**Stance-dance Overpower** (for later — once you're often in Defensive or, at 30,
Berserker Stance). One key: first press swaps to Battle Stance, second press
Overpowers:

```
#showtooltip Overpower
/cast [stance:1] Overpower; Battle Stance
```

Stance-swapping dumps rage above the amount Tactical Mastery (Arms talent) keeps,
so this is mainly worth it once you have points there — until then, weaving while
you're already in Battle Stance is the better play.

## Notes

- **Don't switch targets while the proc is up** — the Overpower window is per
  target; tabbing away (even tabbing back) loses it.
- Non-proc presses show a red "Your target needs to dodge"-style UI error.
  Harmless; if it bothers you, append `/script UIErrorsFrame:Clear()` as a final
  line to wipe it.
- Rage priority: Overpower's 5 rage is checked on the press, Heroic Strike's cost
  is paid on the swing — Overpower first means it never gets starved by the
  queued Heroic Strike.
- Both variants are well under the **255-character** limit.
- Works on WoW Classic Era / Hardcore (1.15) and Classic TBC Anniversary (2.5.x) —
  same macro system. Overpower trains at warrior level 12.
