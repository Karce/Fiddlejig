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

-- Bag/container API. Retail moved these under C_Container; on 2.5.x the globals are
-- current. (Ketho's annotations only ship the C_Container.* forms, hence the gap.)
---@param bag integer
---@return integer numSlots
function GetContainerNumSlots(bag) end

---@param bag integer
---@param slot integer
---@return integer? icon, integer count, boolean locked, integer quality, boolean readable, boolean lootable, string? itemLink, boolean isFiltered, boolean noValue, integer? itemID, boolean isBound
function GetContainerItemInfo(bag, slot) end

---@param bag integer
---@param slot integer
---@return string? itemLink
function GetContainerItemLink(bag, slot) end

--- Spell info by name or id. Retail moved this to C_Spell.GetSpellInfo; the global
--- is current on 2.5.x. First return (name) is what we use.
---@param spell string|integer
---@return string name, string rank, integer icon, number castTime, number minRange, number maxRange, integer spellID
function GetSpellInfo(spell) end

-- FrameXML globals: the WoW API extension only loads the API "Core", not FrameXML,
-- so these come from here.

---@type table<string, fun(msg: string, editBox?: table)>
SlashCmdList = {}

---@type any  # a ChatFrame (MessageFrame); typed loosely so :AddMessage is fine
DEFAULT_CHAT_FRAME = {}

---@type table<string, integer>  # sound kit ids (e.g. SOUNDKIT.TELL_MESSAGE)
SOUNDKIT = {}
