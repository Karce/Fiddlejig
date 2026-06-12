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

## Import strings

Pick the one matching your client (they differ only in the client-version tag, so
the wrong one still works — it just warns on import). Paste into `/wa` → **Import**.

### TBC Anniversary (2.5.x)

```
!WA:2!DrvZUTTrq4OQaKg2FIIsJrBqlGrkIr7fdBvKuGIErurQXfksYK0Xj9NiUKCf5AtTlZUlLSm6LQt(qpPhbDUNeqFb0tWcH(eKd5bWpbz2LsoPvaA58nZo7mZ3oZwQv1Ol(AokusyuXJcgqOerILn8rA5iKio8bLMLGkFjIgMW4T4OHyVjz4a3gonB2zrH6Emcvg0OzhVMoZquYqK(eFK6dATkkNBa9LGxCboKrJelXib2vYX0yzYnMQr6d1MYOyB9gv3YEiIqBP(i1hR(KBO(u1TuvG)x))Rzfkxcjq3mtjy5eYsz8FUm8BUosKc1bG8asS10qglnInM6VCJuZOyCYQni3XKmSF2NTb7Hpt(yIafKIJ8NhHfiPUEWjlgOzcOeqsu5GemjorEOnbIJ)C9QllNhI)YRDFsu2NBJKYu82UjSC52Ftlsuuk(es83UGqhW4fKL1LafI5uu6ZaAcu8pCcDKMXsU8umoRUidhkD07nXoLHIEMtyksi2YzyEQKSf)46ooh01X3wqohVLAhlBThAHajkftLAXz5cCFJF(Z54yimAEx9vZe40bMBr1nNlYdCm2e)WwQkRayak80yolNgDXDwvCL)8UdgiWYsRHVynCzbSF26ZIZKaDPPr7oD70m7osqSpCPX0LD4XKizIDDao3y4DxFxAWdiNHJmB6pNvOGrLzvAXjNV9H5Oin9VTN3QRS5cf)dEhuxD8Uh51(GonxA0EsUqsgmrDZxBGxv321pYRBwLcLji4YVHjxkvQu5SQVN6nf()r5MYVij1l3)bzfbCerqG2h)IKAmJhDmhLn941cQkCGG12EJDCkBS)C9QnyfZtMPLBCfNSudF86bQYR0OwC8RYX0Wj9U(E7w7HgNBBgRURX52ekwCBJOBi0g0R8(7(Dgp9siHNcgfLNAGAMQEd3eWHfAfRRZ)4Ag0l2GkYxTQLqV0pTj)sMlzHJk6C)WEFXujNehdWxUdFT4BMHGmNctnI7T89NiweYggar9PmOKfjSXDPyyuliFWats9KMT716O2o4rqtCWtWOuzYkO7jmblCnBFvHxpLieeASJjgwCy8TlnDI)uyoin9GiH1RHm3Ck94yOZYXTxZ2TVsPleqYzl63OURxFxV6oEQko6CUMDo8GyqwkAcMpxl38mIqks4ad0bcM)SC66Q0Ah1VCXTv)69u)M63vVu1x5RqQav4YICSo8w7iScBPg4xJJhE3fWsxOkrsgV6pQITujkI6e1PWRBPQHkQILOY8dGq5Gh6RELv2TrMdXRiIAA7VV2IiI4KCQXqneDsTCs0LTo5ye32qY7T3E7NvfoKgRFzdCdEmb77mwpDDi)S1DW8jfc)1nSpNXgwQ6WQrvfbpC3A7V73xD0)(83o
```

### Classic Era (1.15)

```
!WA:2!DrvZUTTrqy7Qa4g2FISsJrrqlGrkIrlqHHTkskqrViQi14cfjzsL4K(texsUKCTP2Lz3Lswg9s1jFON0JGo3tcOVa6jyHqFcYH8ay0hGo7sjN0kaTC(MzNDM5BNz3SzLWl)cokqsyuXd9JiuIiXYg(iTCeseh(GsZsqLUcrdsy8MC0aCVXzyF36onA0EEH6Umcv6xVr7EnCMIOKbi9j(q171Czyo3a6lbV4cCaJgkwGrcSRKJPXYKTMOr6d1MYOyB9gv3YEaIqBQ(a1hQ(OTuFS6wQYW)B8)1SeLlHeOtMPeSCcyPm(pwc(nthjsHAFqoIeBnjGXsdzJOElwl1imgNSCnYDejd7L9jRX9WNlFerG8tXHEZcXcKuxp4K5rAMakbKevYpbtItKhBtG44ntV6YY5b4pBJ7rcZ(uBKuMI31nHLl39lBscdtXNsI)Q5eAeJxqwwxbuiMtrPpdOjqXFXj0HAgl5QZW4SAImCG0rV3e7ugk8zobPiHyhNb5PsYo8tQ54CuhhpBb5c8oQ9SS1EOf8LOumvQfNMlW9n(5nJJJHWO5D1NpvGtJm3IQBotK77ySj(UDuLxcqFuWzXCwon8YBVS4k)5DIIey5MRGVyfCrbSF2QZIZKaDPPr72DA3i72sqSpCPX0LDWjKqzIDnaoZy4TxFxzWrKZXHMn97tluWOYSYn5Kl294CuOM(3TxVLxBZfk(7)wOU64DEAVwh1UXcJ2tZfss0y1nFTbEDDBx7P96KvUqzccU8RBYLn3CZszvEh1Rl8)JY1LFrsQxU39Zkc4qIGaTpEfj1igp8eokBYjReuL5abRT9g74u2iVz6vBWkMNmvlx)AozHg(OvduLwQrn54xLJPbJ7EJd2V6dmo3YmwDhJZTiuSyBJOBa0g0T0H7)ngp7LqcodmkknXa1mvT6UjGdZ1kwvN)2gg0lwJkYxTQfqV0pSo)sMjzbdl6CF)V(FMi5K4ya(Y94ReFZueK5uyQrC3fV7eX8a2aFiQpHbLSiHnQdfdJA(5rrMK6XnA1T5tB5GhcnX(pgJsLjlHUNGeSW1S9LfE9eIqqOXoMyyXHX3o00XEtG5G00JcfwVgYCZP0LJHolh3UnA16ALUqajNpVF9AU9672RMtpvzhDox1ohEq0plfngZNPLBCorifjCGbAdbZBAoDvvATN6NUCB1pFx1VO(v1lv9vEkKYxfSOihRbV1oeRWwQiVQC8G7mhw6avjsY4v(EvSLkrruNQodEDlvnqrvSevMNpekh8ap1RSY2gzoKEfrutB)5gZdjItZPgdvr0XvZjHx180tqCBdjFWbhCywf4qQV6LnWn4XeSNZi901X8Zx1bZhxi8hBzFbJnyZkdQewr4)G9RE4(FBLH)9Z)3d
```

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
  Negative X/Y offsets (−7) pull the sparkle orbit **inside** the icon face instead
  of tracing the border; 1.3 scale keeps the sparkles chunky.
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

- Built for TBC Anniversary (2.5.x) and Classic Era (1.15) — same aura, per-client
  version tag. Matching by the localized name `Battle Shout` assumes an English
  client; on another locale, retype the buff's local name.
- Generated and verified by
  [`tools/weakauras/generate.lua`](../../tools/weakauras/generate.lua).
