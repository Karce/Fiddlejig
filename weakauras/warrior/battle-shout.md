# Battle Shout (Warrior)

**What it does:** keeps Battle Shout up with two cues in one icon:

- **Counting down** — in the **last 10 seconds** before it expires, the icon
  appears with a cooldown swipe and a number ticking `10 → 1`, so you can refresh
  it before it ever drops.
- **Missing** — once it's actually gone, the icon stays up (no number) as a plain
  "recast it" prompt.

When Battle Shout is healthy (more than 10s left), the icon is hidden.

## Import string

```
!WA:2!DrvZUTTrq4O6c4g2FIIsRrBqlGrkIrBpe4yGMEPxeDPqCRSOmjTLt)jIl5UKCDO2Ly3LYw(w9jFON0JGo3tcOVa6jyHqFcYH(a4NGolPKtAfGiNVz3DM57BgUn60cF9xkqXkkNjFwucLrLzw2WlLLNuHeWluErgAJBqS4mUOJanIemPGe5VVNJtV51U7ZPmv0(o9cC8MIy0riteFM(D6SexkQadvWPessmNHLliijXxjiSuv2MxzqMGAZ4mITzJ67zpcrzD0VV(d0F4M6psFpDt4)7()9SevQGcWTOIcwEX8CU4h3a(nZKjAT7iWoHMADvmNNJ5NZcxS2YbNsYwUg5FoTGew8XRXbKlu)avIIYj4WzyIePm8HKnpXOeafqk0grzeAAM6iBkKNWzMN(8srm5ZVZJO4Ip1gPu5KT9Z4LQT)QoumoNCgn9RNtzjCrTyzDdiHebdLFcitGJ)sqzJnkw2nVIqkAlliXkpZEZSZ5i8jEX5iPClVrL5k6wIbT98oW1l0wsVKSLEhlBZjmgrkuoHPmMtlLKHvNlCMGKcPXO76VyQKKNu1f13DMSmYRAn53SLU5sagHIFvQGxYWx)GL1T8tDtsKevJvWxScUOgoSyvSeCfixgz0UNBpNIhOaZHqtJBOD8akwLz3gGZQw4nTVBQWj0li4Qn97tRDWzQIMDe0l3(Ose2i)BheS8218bY)43anSt4ECq3d65SOY7zLsfnzI(UVUcElVTBFCGBrZANziO5VFvT0OrJnkA9wUxt8)JZ10VUinpE0JlQt4yQKcJpH1f15CbEGavC1Gvg6McqGnRntXJhx37FV(F2vkbnnfGVChXkZ)zkcg(yWCN8HlE7zQ5X8rrug5qogoqg)CxgbgwJktsQeGN70TFNJ76rgdJbrpNGYvzlb9poJi9R2(Y6tDivkPSuVQCyjGpaCz5tcVcMKYZpalTEnuRvrPVGa9gp)(oD7ERtFiH0lMpC)2(bd9dA7fOB6zQ59SlHRuIkYrtiIzgBNlOsLmtaZJ9GKfoTKTILw7O)PRVVU7d1hQ7PD191hP90(6Gf11yB42QXe9jw6bH7jiJ(K5WdxGLifx0671NAPFH(N1)I(xH7h(n9l1d1HzAuyeKkpYOqDKvX9rvbjOoJgz7pVZCmvEwjRAH9qSj7vsX305SbiHDLiV7U7(0Iwqq2F1DdWXGphjHENBMppsCXQzaXKAJ)yt7l58rnAnQfULm6BFYEp9jFxRX)9P)7d
```

Paste into `/wa` → **Import**.

## How it's wired

Two **Aura** (Buff/Debuff) triggers on the same buff, combined with **Any** (OR):

- **Trigger 1 — missing:** Player buff `Battle Shout`, **Own Only**,
  **Show On: No Aura(s) Found**. The static "it dropped" prompt.
- **Trigger 2 — expiring:** same buff, **Show On: Aura(s) Found**, with
  **Remaining time `<` 10**. This one carries the aura's timer, so the cooldown
  swipe and the `%p` text count down.
- **Display:** a centered `%p` text sub-region is the countdown number (the icon's
  built-in cooldown number is turned off so you don't get two). The number is blank
  during the "missing" state because that state has no timer.
- **Load:** Warrior only, not combat-restricted — so it can remind you before a pull.

## Tweaks

- **Change the warning window.** Want 15s instead of 10? Edit **Trigger 2 →
  Remaining time** (or change `remThreshold` in
  [`generate.lua`](../../tools/weakauras/generate.lua) and regenerate).
- **Make it flash too.** **Display → Glow**, or a **Condition** on *Trigger 2 active*
  → Glow, if the swipe + number is too subtle.
- **Only nag in combat.** In **Load**, check **In Combat**. (Off by design so you
  can pre-buff.)

## Notes

- Built for TBC Anniversary (2.5.x). Matching by the localized name `Battle Shout`
  assumes an English client; on another locale, retype the buff's local name.
- Generated and verified by
  [`tools/weakauras/generate.lua`](../../tools/weakauras/generate.lua).
