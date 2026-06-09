# Warrior WeakAuras

A small set for a leveling warrior, focused on the two things you forget mid-fight:
keeping your shout up and keeping a bleed on the target. Each is a single **icon**
that **counts down in its final seconds** so you can refresh in time, then stays up
as a plain prompt if it does drop — and is hidden when there's nothing to do.

| Aura | Counts down… | Then prompts when… | File |
|---|---|---|---|
| Battle Shout | last **10s** of the buff | it's **missing** | [battle-shout.md](battle-shout.md) |
| Rend | last **tick (~3s)** on the target | target **lacks your Rend** (in combat) | [rend.md](rend.md) |

See the top-level [WeakAuras README](../README.md) for how to import a string
(`/wa` → **Import** → paste).

## Design notes (shared by both)

- **Countdown + "show on missing."** Each aura pairs two triggers with **Any** (OR):
  one shows the icon with a counting `%p` number in the buff/debuff's final seconds
  (refresh in time), the other shows it as a static prompt once it's actually gone.
  Between those, when everything's healthy, the icon is hidden.
- **Own only + match by name.** They track *your* aura and match by spell **name**,
  so every rank counts automatically as you level and train new ranks — you never
  have to update a spell ID.
- **Class-gated.** Both `Load` only on a Warrior, so they sit dormant on other
  characters that share the account's WeakAuras.
- **Placement.** They import stacked just below center screen (Battle Shout above
  Rend). While the `/wa` window is open you can drag them anywhere; the move
  handles only show while options are open.

Want a glow/flash or a sound instead of a plain icon, or a warning *before* a buff
falls off rather than after? Each file's **Tweaks** section covers it.
