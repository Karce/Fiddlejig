# Rend (Warrior)

**What it does:** keeps your Rend bleed rolling on the target, with two cues in one
icon (only while you're in combat), using the gold **Autocast Shine** glow (the
sparkle WoW puts on autocast-enabled buttons) to grade urgency:

- **Counting down** — when Rend is down to its **last tick (~3 seconds)**, the icon
  appears with a cooldown swipe and a number ticking `3 → 1`, and the glow is
  **downplayed** (dim, small) — a gentle "it's nearly up" heads-up.
- **Off the target** — once Rend has actually dropped, the icon stays up (no number)
  and the glow goes **full** (bright, full-size) as a strong "reapply it" cue.

It hides when Rend is freshly applied, and whenever you have no target.

## Import string

```
!WA:2!DrvZUTTXt8OQa4g2pIIsRrAqoyKI4)TxeCCQDlc6frhPghiljtQuNE5V4sYLKRn1Um7UuF4EQ6uo0t6rqN7jb0xa9eSqOpb5qEa8tqNDPKtsfGwo)MzNVNzl1SA4B(wokqsyuXH(rekrKyzdFKwocjIdFqPzjOYxHObjmEtoAaU3KmSV7ronA0Erb7Umcv6FuJ29A4mdrjdqAlEO6tAUkmNBa9LGwCboGrdflXib2vYX0yzYwt1iTrTPmk2wFr1TThGi0MQpt95QVyl1xQUTQc8)M)xoRq5sia6KzsblNawkJ)IYWV5AprmSV)U8GeenglE6UtZ4miqaNuvK7x7j1Yf4FjLn6iTIodrP54KDvBFvHW4nsu3Zy0pwIBakfRUxzNGeCWfhYLCsCmMxgynDiItq(PyBrcBKpelrKyRPbmwAiBe1B5gQgHX4KvBqUJizyVSVAdUhES8zeH2qHEZdXcKuxpXjlI0DcOeIKOY(jysCI8uBc4hV56txwopa)GB8qsywfhmnCNVRjjmmfFoj(7xqOrmErtY6kO1H5uu6VcThGXFZj0H6ovYvxGXz1fz4aPJ(Uj2PmuyKtqksi22zqEQKSn)S6ooh3XXZwqUeVTAplBTgAcFjuEOsn5mOk33ON3CdjBGps6nNJJbxQ79Q)3mbonYmjPU1COi7yKjE62QkRaOpk4IyolNg(M7UQyS7vDIIeyzP1WFBnCzbSF2ABXzsOKPlL2T70Ur2DLazFyWHPlbbNrcLj21b4CJG3pcDLbhrgJdnx6pMvWGrLzvAYjxUZP5OqDlyNE9wDTmxOq8O3d1zhVZl71642nwA4EEUqsIMOU1BnWRZB76VSxNSkfmtqHRN9ErPsLkNv9dyVjX)iMBs)IGuF8WhLv4WHebbgH8kcQrmE4zCu20ZwtOQWHcSw27S1Z1EZ1N2Gumpz217aVOC3BUxTF6GUL3R2t0K)Wbl1cF26v8YR0OMC8RZX0Gj6BS)bgt1YSO)1gt1IqXI7m769h91o8aJQ9sibxasfLNAG6cx9JCtanwOzSjTnGnPBrWR5S8dxL9MlzbdlgP)0UFZ017MI))UQh8UziiKPWcK4(26LJf65rWlNWGmwVW2HIHTn)8Oitq886oN08LTCWdH5z)NJrPYKvWWdS3lCnxFvHwNqeccn2XyBloSb3HMoXBkSsKMECOW6TqSASsxoggSCC72OvRRz6coKmEr)JQ72RVBV6o9uvC0X6(25WBYWcfpglNRPBmMiKIeoKZTbN5nlNUofTGm8n3rHVVksfRsue15QluPQbllIX6WZ9dXkMLkZBFoEWwlGJoqwIKmE1Fw9Alfxjusvo8a7q1i1y1Ke1LE(GRCWd8u)Uv2DqgJ0RWJ6Y2FDJfHeX55uJG9r0j7NtcVQ55NH46s8E6FpoRkyKJw)4gOg8Uc2ZzKE56u(41Du(KcI)0Y(sgBqPQdQgwv4FqT9FCTFS6W)5v)7d
```

Paste into `/wa` → **Import**.

## How it's wired

Two **Aura** (Buff/Debuff) triggers on your Rend, combined with **Any** (OR):

- **Trigger 1 — missing:** Target debuff `Rend`, **Own Only** (so it tracks *your*
  Rend), **Show On: No Aura(s) Found**, **Unit exists: off**. The static "apply it"
  prompt — and the "unit exists: off" is what keeps it hidden when you have no target.
- **Trigger 2 — expiring:** same debuff, **Show On: Aura(s) Found**, with
  **Remaining time `<` 3**. Carries the timer, so the swipe and the `%p` text
  count down through the final tick.
- **Display:** a centered `%p` text sub-region is the countdown number (the icon's
  built-in cooldown number is off to avoid a double). Blank during the "missing"
  state, which has no timer.
- **Glow:** an **Autocast Shine** glow sub-region, set **downplayed** by default
  (dim gold, 0.65 scale) — that's the look during the final-tick countdown. A
  **Condition** (*Trigger 1 (missing) Active* → full color + 1.0 scale) promotes it
  to the full shine once the debuff is off the target.
- **Load:** Warrior, **In Combat** only — so it won't flash when you click a
  friendly/quest NPC out of combat.

## Tweaks

- **Change the refresh window.** Rend's tick interval is ~3s; if you want an earlier
  warning, raise **Trigger 2 → Remaining time** (or `remThreshold` in
  [`generate.lua`](../../tools/weakauras/generate.lua)).
- **Show before combat too.** Uncheck **In Combat** in **Load** to get the prompt the
  moment you target an enemy. Trade-off: it'll also show on a hostile you're only
  inspecting.
- **Skip it on trivial mobs.** There's no "is this worth bleeding" check — just glance
  past the icon on quick kills.
- **Tune the two glow states.** The downplayed look lives in the glow sub-region's
  defaults; the full look lives in the **Condition** on *Trigger 1*. Adjust **Scale**,
  **Color**, or **Frequency** in either to taste.

## Notes

- Rend needs a melee weapon equipped to land; the aura only reports the debuff, it
  can't tell you why an application failed.
- Built for TBC Anniversary (2.5.x). Name match assumes an English client.
- Generated and verified by
  [`tools/weakauras/generate.lua`](../../tools/weakauras/generate.lua).
