---@meta
-- Classic / TBC (2.5.x) API that Ketho's *retail-focused* annotations don't cover.
-- Keep this minimal: only globals this project actually uses that are missing or
-- wrong upstream. Signatures from warcraft.wiki.gg (Classic). See docs/api/README.md.
--
-- This file defines no behavior; `---@meta` marks it as type definitions only.

--- Pet happiness. Removed from Retail in Patch 4.1, but present in Classic/TBC.
--- Drives the pet's damage modifier and loyalty gain.
---@return integer happiness        # 1 = unhappy, 2 = content, 3 = happy
---@return number damagePercentage  # current damage multiplier from happiness
---@return integer loyaltyRate      # per-tick loyalty change
function GetPetHappiness() end

--- A buff on `unit` by index. Retail replaced this with C_UnitAuras/AuraUtil; the
--- global is still the way on 2.5.x. `spellId` is the 10th return.
---@param unit string
---@param index integer
---@param filter? string
---@return string name, integer icon, integer count, string debuffType, number duration, number expirationTime, string source, boolean isStealable, boolean nameplateShowPersonal, integer spellId
function UnitBuff(unit, index, filter) end
