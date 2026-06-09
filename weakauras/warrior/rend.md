# Rend (Warrior)

**What it does:** keeps your Rend bleed rolling on the target, with two cues in one
icon (only while you're in combat):

- **Counting down** — when Rend is down to its **last tick (~3 seconds)**, the icon
  appears with a cooldown swipe and a number ticking `3 → 1`, your cue to refresh
  before it falls off.
- **Missing** — if the target has no Rend from you at all, the icon stays up (no
  number) as an "apply it" prompt.

It hides when Rend is freshly applied, and whenever you have no target.

## Import string

```
!WA:2!Drv3UTTnu46McKvTFQR7qawXgqqhAW2UOynaR7MDJvMnAgsSsOuMt3p1IsKsIPYKcKuoX5U5RYf7k)i4R3vgyVa(jGWypb9I9a4NGDOKD62eGLoFhsE(578z2OBlYnFUehRzcU6frjmotL54cF0oiLglHp48Im8wRW84mHSRepKgmUGg5FaQtNEZRDFIGX1rh0Pxqh0umNneBJ4lm3T7ssPScmqdNsQOXcorTGIvuFTKYt1zBpXISb1Ll4ux7gnpWDiMX7AEFZhy(WTnFK5bMMWV79)9SexQHcWROQfCqXICH8h2cEMzZeR2DeyNWsDMele5eXL8WfBS6qsPzl3G8VKvqdl(4n4a6v6VNPWr5us4mcvH12(HMnpXYeqlG14TIYOS0m9PUmipHZSV9fLYy6NENNWifnruoz3VOlJqYPxWs)Y5mEIqwtsoRaQJk548FeOhWXFkz8rwMkB1BO0I2QcASgz3BMBUatsqX5yLAh0WYCnBhz)2i0HEOqxf7A6oM9CCTNWAePX5uU2AoTurhuDUWzvMIHryD4mjnfsPL7nF2ufnpPAsAU)mvzeQAn1xTJP5sagHJFtQuuYj38OL1J9Z9ssuuDJ1WxTgUOgoOyDSKcnqzwQ0TNxVofpsdMdGbNWsbX9zeDMBBaoRAH3ncxvHtyxrjvB63Mw7qW1fn7kzxV7PLyIDeSBqWYBxZhiIN(oOT7KENfC0H96SOY7fLknlzS5(VTcEBF72(SaVIM1oZWGa4GQAPrJgBv06F5EtJ)FCUP9Rls7RN80I6eoIPyGekSUOUuij9L4Ij9xByAkbc2U2mTiEuTo49o5tMOLS0ua(69KRn)7PyqaYbTN6XUwD1C7OKXPhliWgZex6XPGqnQmjPQXFzB0XDp7ieDeifIEjfNRZwc8ECgv5xT9L1N6yMsX4POQy7ibXVhpFC4eqnLNFir58wOgRIYjskmtq(N05OJU1PpKq2vZhCqB)Gb(bTrbMMiBTUVBjCDcOfLPu9mRDNRykTktccXEqYcNwYx3Do7zo6MhAo(XMEgpZjMtniJVjWC2I6ASnCt1iQPVJ58W9L0HBphE5bDjwlKT(oZRCm)K5Nn)I5xH7gETzGj0GZmrHrqQq0HHMyNIhIRcsqDgT02FCN5eM6IsE1c7J5J3VKrw19I(yPLI)A7ZZlAbb5G13lahd(ljneDPvxEQ8Q1ZE54AJF3X9AHyyJwdBrAPI(MNT)ZF232A0FD()8d
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

## Notes

- Rend needs a melee weapon equipped to land; the aura only reports the debuff, it
  can't tell you why an application failed.
- Built for TBC Anniversary (2.5.x). Name match assumes an English client.
- Generated and verified by
  [`tools/weakauras/generate.lua`](../../tools/weakauras/generate.lua).
