# Macros

Macro collection, grouped by class or category in subfolders
(`hunter/`, `general/`, ...).

Each macro lives in its own Markdown file with three parts:

1. **What it does** — a short description.
2. **The macro** — the body in a code fence, ready to paste into the in-game
   macro editor (`/macro`).
3. **Notes** — caveats, character limit notes, variants, required conditions.

## Rules

- WoW macros are plain text and **limited to 255 characters**. Keep the macro body
  within that limit; if you can't, it probably needs an [addon](../addons/)
  instead.
- Prefer macros over addons (see [`../docs/conventions.md`](../docs/conventions.md)).

## Template

````markdown
# <Macro name>

**What it does:** <one-line description>

```
#showtooltip
/cast <spell>
```

**Notes:** <conditions, variants, character count, etc.>
````
