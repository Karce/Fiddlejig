# Battle Shout (Warrior)

**What it does:** keeps Battle Shout up with two cues in one icon, both wrapped in
the gold **Autocast Shine** (the sparkle WoW puts on autocast-enabled buttons):

- **Counting down** — in the **last 10 seconds** before it expires, the icon
  appears with a cooldown swipe and a number ticking `10 → 1`, so you can refresh
  it before it ever drops.
- **Missing** — once it's actually gone, the icon stays up (no number) as a plain
  "recast it" prompt.

The glow is on the whole time the icon is shown (in both states). When Battle Shout
is healthy (more than 10s left), the icon is hidden.

## Import string

```
!WA:2!Drv3UTTnu46LcKvTFQJ7AWwXgqqhAW2nbPgODad7glx71m4A7iP009xTOKOKyImPkjLDCUB(QEXUYpc(6DLb2lGFcim2tqVOpa5jyhszN2ndyQZ3H8WZ57J8WkTRf96VMJcLegv84GycLisTSHpslhHeXHpOS8u0wxHOHPmEBoAi2BsooWTPtRwDxu6UpJqLbnB11RLZmeLmeP3XhR(G2RIk4gWajefxGdz0iXsmsGDLCmnrMU9unsVP2ugfBRxO622dreAB1hP(y1NST6tv3wvf(FZ)VNvOcjua9YnuWYjKLX4)0wWV56mrkDha2XKeRPHmwweBm1F5gRwrj40vBqUJj5y)8pBd2dFH8jebkidh5ppclqsnFWPlI1kbqbKeTvqkMKKkp2Ma5XFUE0LvWdXF5nUpjk)ZTrszgEp3uwHCVVPnjkkdFgj5BxqOXmEPyzDfiHyofL9CqMah)nNqhPvS0RohJZBiYXHsh9AtTZyOON7eMHeIDDgwKjj7YpTHJZr9C8TfKlX7Q23YwhH2iqIYWuP2CwHapWeN)CoobsJw3vF1mbol2CkQU1CrrGJzoX3VRQ6kagGcppHZkOrV(oRkpYFrV4ybwwzn8NxdxwchKVEV4mjixAz0UBVUTYVJemhahAmnTdpLejtTBaW5MjE3X3vgCm5cCKzr)XSshmQmVABo5Y9oUafPL)988wD9CUa5FW7GA2X7DIxNJ62APX7zfcjjEI6wVXaVM32noXRxE1sNPi4WVPPwQuPYw51Ep3Bi()X5g6xwK6H7)G8YeoIiiW1h)YIAmJhDkhLp901gQQCqG1Z9w7Km2y)56rBywmpDM2U51AYsn8jRBO2ALg1MJFvbMgoP)np8G6pYeChtB1Dnb3HqXIDmMUHW1GYO8sjHNdti2AQbQvPgnDtHfVq7ydhnGnCRSs1Ewc3I(Xnvw6CjlCu5D2pS)xmvYjjja8L7ZxB(2ziOMPq)I4ElF)EHfHSHbqoFgdiRiLnUhfdnzbfXXMs6PT60V9jDCWJGRVbpfJYKPRG7nHPyHRz5RkJ6zeHGqtCm5WIdnU9Ozt8NcDazzhfjSEdu5MDPphd3PCC73QtNRD6cjKCXIbnB46nW1RHJNQQJUMRBxapfgKNHMG5Z12TUGiKIuoOaDHK5pRGUMLw7R(LxVJ6xVN63u)U6LQbkFfsfOcxwwJnGxzhHvylvSFDoE4Dxad9awIKmETFqLyPsve1zQZH31YudvuflvL7haPYbp0x9kR8DqMnXRmJAz7VUXIiI4ScQzI6i6K6fKORAF2PiUTrKp8WdFyEnytAU(nnim4zeSVZyDF1X8lwF(YNuA8NBBFjJnSsTH1IQjcE0b1F4bFxTr)Zl(3p
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
- **Glow:** an **Autocast Shine** glow sub-region, on by default — so it shines
  whenever the icon is up. No condition is needed since both shown states want it.
- **Load:** Warrior only, not combat-restricted — so it can remind you before a pull.

## Tweaks

- **Change the warning window.** Want 15s instead of 10? Edit **Trigger 2 →
  Remaining time** (or change `remThreshold` in
  [`generate.lua`](../../tools/weakauras/generate.lua) and regenerate).
- **Glow only during the countdown.** If you'd rather the shine *not* show in the
  fully-missing state, add a **Condition**: *Trigger 1 (missing) Active* → set the
  glow sub-region's **Visibility** off.
- **Tune the shine.** The glow sub-region exposes **Scale**, **Frequency**,
  **Lines & Particles**, and a custom **Color** if gold isn't your taste.
- **Only nag in combat.** In **Load**, check **In Combat**. (Off by design so you
  can pre-buff.)

## Notes

- Built for TBC Anniversary (2.5.x). Matching by the localized name `Battle Shout`
  assumes an English client; on another locale, retype the buff's local name.
- Generated and verified by
  [`tools/weakauras/generate.lua`](../../tools/weakauras/generate.lua).
