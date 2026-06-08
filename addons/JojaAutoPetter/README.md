# Joja Mart's Auto-Petter

A Fiddlejig addon that keeps your hunter pet happy with the least fuss. A nod to
*Stardew Valley*'s Auto-Petter (the JojaMart gadget that pets your farm animals so
you don't have to) — here it auto-picks the **best food in your bags** for the pet's
diet and feeds it with one keypress, and reminds you when the pet gets hungry. The
plan is to grow it into vendor-buying and auto-cooking too; see Roadmap.

## Why an addon (and not a macro)?

Per the project's macros-first rule, here's why a macro can't do this. To pick the
*best available* food you have to:

1. read the pet's diet — `GetPetFoodTypes()`;
2. loop your bags and evaluate each item's level against the pet's;
3. feed a **dynamically chosen** bag slot.

Step 3 is a protected action: it can only run through a `SecureActionButtonTemplate`
whose attributes are set by out-of-combat Lua. Loops, API calls, and secure-attribute
logic are all outside the 255-char restricted macro environment. A macro can only feed
a *hardcoded* food, e.g.:

```
/cast Feed Pet
/use Roasted Quail
```

So "feed the best food automatically" is a genuine addon case.

## How it picks food

Happiness per feeding pulse depends on the level gap (`petLevel − foodLevel`):

| gap (food below pet) | result | happiness/pulse |
|---|---|---|
| ≤ 10 (incl. food ≥ pet level) | Loves | 35 |
| 11–20 | Likes | 17 |
| 21–29 | Eats | 8 |
| ≥ 30 | **Refuses** | 0 |

It selects: **highest reachable tier → lowest-level food in that tier → smallest
stack.** That keeps the pet fully happy while *conserving* your better/pricier food
and using up partial stacks first.

## Feeding

There's no visible button by design — you trigger it:

- **Keybind:** Key Bindings → *Joja Mart's Auto-Petter* → **Feed Pet (best food)**. *(Recommended)*
- **Macro:** `/click JojaAutoPetterButton` on your action bar.

Feeding is out-of-combat only (so is the pet's appetite), which is why it's a keypress
rather than fully automatic — Blizzard doesn't allow an addon to feed for you.

It **won't double-feed or waste food**: the key feeds once per press, and the button
is disarmed (so a press does nothing) while the pet is still eating its last meal
(the *Feed Pet Effect* buff is up) or is already **Happy**. It re-arms when the buff
fades or happiness drops. `/joja debug` shows both gates.

## Reminder

When the pet's happiness drops below *Happy*, you get a one-line chat nudge (naming
the food it would feed) and a soft sound. One nudge per state change, never in combat.

- `/joja` (or `/autopetter`) — show the current best food + reminder state.
- `/joja off` / `/joja on` — toggle the reminder.

To only be nagged when the pet is actually *unhappy*, set `REMINDER_BELOW = 2` near
the top of `Core.lua`.

## Food coverage

`Foods.lua` maps `itemID → diet` for TBC foods (Meat, Fish, Bread, Cheese, Fruit,
Fungus). Only the diet is stored — a food's level is read live, so the table never
drifts on levels, and a missing/wrong entry is harmless (the game refuses food the
pet can't eat). Add IDs as needed.

**Cooks:** raw meat/fish (cooking mats) count as food and may get auto-fed. To save
some for Cooking, add their itemIDs to `ns.Exclude` at the bottom of `Foods.lua`,
e.g. `[769] = true` for Chunk of Boar Meat.

## Roadmap

Planned provisioning features (tracked in the repo `TODO.md`):

- **Auto-buy** compatible food the pet loves from vendors, toggleable on/off.
- **Auto-cook** raw food in your bags when Cooking is known, in as few clicks as possible.
- **Low-food alert** when you're running short on suitable food.

## Icon

`Icon.tga` (64×64, true-color uncompressed) is referenced via `## IconTexture:` in
the `.toc`. Heads-up: `IconTexture` is a Retail 10.1 TOC field and the stock Classic
2.5.x AddOns list predates per-addon icons, so it may not render there — it's
included in case the Anniversary client (modern engine) honors it — and it does show
in-game. Regenerate from any square-ish source with ImageMagick:
`magick src.png -alpha on -background none -gravity center -extent 96x96 -resize 64x64 PNG32:- | magick - +map -compress none -type TrueColorAlpha TGA:Icon.tga`

## Install

Copy this `JojaAutoPetter/` folder into your WoW install's
`_anniversary_/Interface/AddOns/`, then enable it on the character-select addons
screen. Requires a Hunter (Feed Pet, level 10+).
