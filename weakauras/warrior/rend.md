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

## Import strings

Pick the one matching your client (they differ only in the client-version tag, so
the wrong one still works — it just warns on import). Paste into `/wa` → **Import**.

### TBC Anniversary (2.5.x)

```
!WA:2!Dr1tVTTnu845cKvT)56Uf0v0dfDOzBxmstwYgk2fRu71eKg7iPU0EzrususmrMuLKYooBxMp1d7K)i4Z7Kb2xa)jGWyFc6H(bWFc2Ju2PTZaM6979iF)JVFSs76rV(B4OqjHrf7fetOerQLn8rA5iKio8bLLNIQUardtz82CupS3WCCG7(oTAD80s1DzeQmy)wh71YzmIs6H0ECp1h1EEub3aotcNIlWHmAKyggjWUsoMMitxFKgPDQnLrX26nQ(c7EicTT6tuFQ6Zwx95QVqvd(FJ)VM5OcjKaDYnLGLtilJXpSk8BIoseJ67UjpmfrtWIhV5OCodseii1ffbn2PrHa)lzSb7RpOtFuwboDt1glknMSYI6ogN(HwCdrzy1D6w9rn2XjmfhEXECjNKKG5vv3P6O(iobfKHTfPSbbq(etsSgfYyzrSbu)zRKAfLGtNVc5oGKJ9Z)Yvyp8LYNqeAhf5pjclqsDpfNonwFBaTrKevniftssLNytG44prV6Yk4H47T2dir51CW0O7)DTjrrz4ZjjF)ucnMXlVOSwaxFyofL9RWveO4F4eAF9Tv6IlW48MICCO0rV3u7mgkk2jmdjeB40Rits2GFAthNd644BlixH3qTLLT(eAHaj0IOsT4yOtFM5C(tmISEbiP)eoobcP((x9TJf4SyZ0K6MtGgTJXM4XBOQnhGbOWls4ScA0RV98YrVx0jowGLvwcF5s4Ss4z5l9fNjHwMUvAFCNJBLFBjiEgm8W0TGWtjrYu7MaCIXW7gJwyWXKlXrMn9NJlvWOY8AT5KRU)jfOi9vW998MFTnxOr8W3b1vhVZZ9o6GJBnZO98cHKepuDZ3yGxx32nFUxN8ALktrrlN)oSsLkvZR)EQxv4FGYvLFzsQxEWdZldyFIGaJq(Lj1agp6uokF0Plfu14qdwB7T26zB)j6vBWkMNo(AEWHv7EJTA8t72T6wn2rl(d7otB8jlP5vNRrT54xvGPHd17y7DnU6idz)RmU6icflU14R5qL(0CuVus4fGvr1rgOUX1CF3u4et1kww2)XAg0lxHktFTQzVpH2FIKf2VCO(J7(1JwYof)2MQ792XiiPPafsCxBn9yQEIeIZZyqnRPSDOyGVfuehBsJN205zTF(ro4(WeDWtXOmz6Cy8by(cxZ2NxEQNreccnXX4BloWH7qZg6pcifzzhejSEdKRgV0LJHrlh3UTo6ORv6cbKC50Z2VPR3zUEnD8u1C05622fWlZaLINGLt0YTUKiKIuouZhdbZFCbDzjAbv4RVLcFxvSkrLQiQZvxOYu9MvMJnHh97JvmlvU)2CCV1NclDGQejz86)S6vwkUsOKQc4z2(QbQlvdtvx5haHYb3Zx97w53czCIxze1TT)ETPreX5fuJHTr0HBxqIw0(8trCDlEl9VhLxhCY(lFEdog8Yc23zGMEDc)YLJW8HLc)LL9vmwVk17vpQUiy3gB)Og)y9()7l(Vp
```

### Classic Era (1.15)

```
!WA:2!Drv3UTTnu465c0vTFQJ7wqrrVOOdnBdyWiXzjBOy3y5AV2I04uj1L2BMfLeLetKjvjPSJZ2nZx1l2v(rWxVRmWEb8taHXEc6f9biypa7qk702zatD(ohYZF88XkDRh96VIJcLegvSFqmHsePw2WhPLJqI4WhuwEkQ6fiAykJ3LJgG9gNJdCB70PZHZlvFeJqLbT7COxhNPikzas7X9vFu3LrfCdOVeofxGdz0iXcmsGDLCmnrMETjAK2P2ugfBR3O6g2dqeAx1NO(u1NDn1NRUHQg8)Q)Fnlrfsib6LBkblNqwgJ)KQWVz6irmQV9w8WuenblEWwtY5mirGGuxue0y3gfc8pNXg1wFqNHOScC6wQnVO0yYAlQBzC6hAXneLHv36OQ70yxNWuC4P7ZLCsscMxvDRQtgI4euqg2wKYgfa5tmjXAsiJLfXgr9xSwQtucoD5AK7iso2p)lwJ9WNjFirODuK)SiSaj19uC68y9Tb0grsu1Gumjjv(mBceh)z6vxwbpeFNRCpsuEnhmn6UFtxsuug(esY3oNqJz8YlkRlGRpmNIY(f4kcu83CcDO(2k9ItX48wICCO0rV3u7mgkk2jmdjeB6mOits2KFClhNh3ZX3wqohVPABlB9j0cbsOfrLAXPqNUV5C(ZmISbbiP)moobcP((x91tf4SyZ0K66ZGgTJXM4bBQQTeGbOWtt4ScA0RV5YYrVx0lowGLvwbF5k4Isy)8v(IZKqlt3kTpS3HDYVPee7ddpmDli8ysKm1UfaNzm8UXOlm4yYz4iZM(JPLkyuzETUCY539zfOi9vWD98wEPnxOrC)3b1vhV3Z9o4Xh2zHr7jfcjjES66VXaVSUTB9CVE51kvMIIwn)9KkvQunV(7PEDH)bkxx(LjPE5E3pVmGdjccmc5xMuJy8OJ5O8jhVsqvJdnyTT3ARNT9NPxTbRyE60l5bpP6rxD7g)4EhvD7g7Qf)(9wOn(Wv08Ql1OUC8RkW0WX6D0CpJRoWq2)sJRoGqXInMEjhQ0NMJ6LscpfSkQoXa1nUwTDtHtmxRyvz)7xXGE5Auz6RvT49j0(ZKSWHLd1F839VtwXof)6wQ782PiiPPafsCBBn9yUEIeIZtzqnRPS9OyGVfuehBsJh1Y5PDF(bo4HWeDWJWOmz6sy8by(cxZ2xwEQNseccnXX4BloWH7rZg7pbifzzposy9gixnE5ioggTCCpQZbhCPsxiGKZM3VDlxV(UETC8u1C05At7c4LzGsXtWYzA5oNresrkhQ5dHG5pTGUQeTGk81BOW3wfRsuPkI6e1PQm1GfL5yl4r)HyfZsL73KJhCT5WspOkrsgV(pPELLIRekPQaEMDOAK6m14u15(bqOCWd8v)Mv(giJt8kJOUT9xxzEerCsb1yOjIoUzbj6IUNCmIRBXBR)TtEDWjTx98gCm4LfSVZin96z8ZwncZhxk8Nw2NZydQuFq9O6IG9A0CNg)q9H)Zl(Vd
```

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
  (dim gold, 0.85 scale) — that's the look during the final-tick countdown. A
  **Condition** (*Trigger 1 (missing) Active* → full color + 1.3 scale) promotes it
  to the full shine once the debuff is off the target. Negative X/Y offsets (−7)
  pull the sparkle orbit **inside** the icon face instead of tracing the border.
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
- Built for TBC Anniversary (2.5.x) and Classic Era (1.15) — same aura, per-client
  version tag. Name match assumes an English client.
- Generated and verified by
  [`tools/weakauras/generate.lua`](../../tools/weakauras/generate.lua).
