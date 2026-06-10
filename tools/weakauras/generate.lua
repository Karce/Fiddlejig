-- Fiddlejig — WeakAuras import-string generator.
--
-- Builds "!WA:2!" import strings for the auras defined in `specs` below, using
-- WeakAuras' own serialization pipeline (LibSerialize -> LibDeflate -> EncodeForPrint),
-- exactly as the in-game Export does (see WeakAuras/Transmission.lua: TableToString).
-- Each aura is built once per client FLAVOR (TBC Anniversary, Classic Era) — the
-- flavors differ only in the `tocversion` tag, which is what keeps WeakAuras from
-- warning "made for a different game version" on import.
--
-- The aura data tables here mirror WeakAuras' schema for an `icon` region with two
-- `aura2` (Buff/Debuff) triggers, taken from the installed source:
--   * Private.data_stub / region defaults  -> Types.lua, RegionTypes/Icon.lua
--   * aura2 trigger fields                  -> BuffTrigger2.lua
--   * subtext sub-region (the %p countdown) -> SubRegionTypes/SubText.lua
--   * subglow sub-region (Autocast Shine)   -> SubRegionTypes/Glow.lua (glowType "ACShine")
--   * conditions (per-state glow)           -> Conditions.lua, WeakAuras.lua ("show" var)
--   * transmit wrapper {m,d,v,s} + v=1421   -> Transmission.lua (DisplayToString)
--
-- Usage (from anywhere; paths resolve relative to this script):
--   lua tools/weakauras/generate.lua            # print all strings
--   lua tools/weakauras/generate.lua --write    # also patch them into the docs
--
-- The WeakAuras libraries are found via gitignored symlinks (personal install
-- paths never reach the repo):
--   tools/weakauras/.local/wa-anniversary -> <anniversary>/Interface/AddOns/WeakAuras
--   tools/weakauras/.local/wa-era         -> <classic era>/Interface/AddOns/WeakAuras
-- or set WA_LIBS to a WeakAuras/Libs directory to override.

-- This is standalone Lua (run by the system `lua`), not addon code, so it uses the
-- real os/io libraries that the project's .luarc.json disables for the WoW sandbox.
-- Scope the resulting false positives to this file only.
---@diagnostic disable: undefined-global

-- WoW runs Lua 5.1 (global `unpack`); standalone Lua 5.4+ moved it to table.unpack.
-- LibSerialize's decode path needs it; shim so this runs on either.
unpack = unpack or table.unpack -- luacheck: ignore

local SCRIPT_DIR = (arg and arg[0] or ""):match("^(.*)[/\\]") or "."
local ROOT = SCRIPT_DIR .. "/../.."

-- Both client flavors ship the same WeakAuras (verified: 5.21.7, internalVersion 90
-- on both installs), so one copy of the libs serializes for both. Re-check those two
-- pins in Init.lua/WeakAuras.lua when WeakAuras updates.
local FLAVORS = {
  { key = "anniversary", heading = "TBC Anniversary (2.5.x)", tocversion = 20505 },
  { key = "era", heading = "Classic Era (1.15)", tocversion = 11508 },
}
local WA_VERSION = "5.21.7"
local INTERNAL_VERSION = 90
local TRANSMIT_VERSION = 1421 -- single aura, no sub-groups (Transmission.lua)
local DEFAULT_FONT = "Friz Quadrata TT" -- WeakAuras.defaultFont

local function findLibs()
  local candidates = {}
  if os.getenv("WA_LIBS") then table.insert(candidates, os.getenv("WA_LIBS")) end
  for _, f in ipairs(FLAVORS) do
    table.insert(candidates, SCRIPT_DIR .. "/.local/wa-" .. f.key .. "/Libs")
  end
  for _, dir in ipairs(candidates) do
    local probe = io.open(dir .. "/LibDeflate/LibDeflate.lua", "r")
    if probe then probe:close(); return dir end
  end
  io.stderr:write("WeakAuras libs not found. Create the .local symlinks (see README)\n" ..
    "or set WA_LIBS to a WeakAuras/Libs directory.\n")
  os.exit(1)
end

-- LibSerialize captures `pairs` as an upvalue and uses it to walk a table's map
-- keys; pairs() order is randomized per process in Lua 5.4+, so two runs would emit
-- byte-different (though equivalent) strings. Install a deterministic, sorted pairs
-- before loading it so regenerating an aura always yields identical output and the
-- committed strings don't churn. (Array keys are still walked in order via ipairs.)
local realpairs = pairs
local function sortedPairs(t)
  local keys, n = {}, 0
  for k in realpairs(t) do n = n + 1; keys[n] = k end
  table.sort(keys, function(a, b)
    local ta, tb = type(a), type(b)
    if ta ~= tb then return ta < tb end          -- numbers before strings, stably
    if ta == "number" or ta == "string" then return a < b end
    return tostring(a) < tostring(b)
  end)
  local i = 0
  return function()
    i = i + 1
    local k = keys[i]
    if k ~= nil then return k, t[k] end
  end
end
local LIBS = findLibs()
pairs = sortedPairs
local LibDeflate = assert(dofile(LIBS .. "/LibDeflate/LibDeflate.lua"), "LibDeflate failed to load")
local LibSerialize = assert(dofile(LIBS .. "/LibSerialize/LibSerialize.lua"), "LibSerialize failed to load")
pairs = realpairs

-- One aura2 (Buff/Debuff) trigger. `showOn` is "showOnMissing" or "showOnActive";
-- `extra` adds the remaining-time check for the countdown trigger.
local function auraTrigger(o, showOn, extra)
  local t = {
    type = "aura2",
    unit = o.unit,                 -- "player" | "target"
    debuffType = o.debuffType,     -- "HELPFUL" (buff) | "HARMFUL" (debuff)
    useName = true,
    auranames = { o.auraname },    -- matches by name => every rank counts
    ownOnly = true,                -- only auras you applied
    matchesShowOn = showOn,
    unitExists = false,            -- hide when the unit doesn't exist (no target)
    combineMode = "showOne",
    -- harmless defaults carried by data_stub:
    event = "Health",
    subeventPrefix = "SPELL",
    subeventSuffix = "_CAST_START",
    spellIds = {},
    names = {},
  }
  if extra then
    for k, v in pairs(extra) do t[k] = v end
  end
  return t
end

-- The Autocast Shine glow sub-region. `downplayed` = the subtler look used while a
-- timer is still counting down (dim gold, smaller); the full look is bright natural
-- gold at full scale, applied either always (Battle Shout) or via a condition (Rend).
local function glowSubRegion(downplayed)
  return {
    type = "subglow",
    glow = true,
    glowType = "ACShine", -- the gold autocast sparkle (LibCustomGlow AutoCastGlow)
    useGlowColor = downplayed and true or false,
    glowColor = downplayed and { 1, 0.85, 0.3, 0.45 } or { 1, 1, 1, 1 },
    glowLines = 8,
    glowFrequency = 0.25,
    glowDuration = 1,
    glowLength = 10,
    glowThickness = 1,
    glowScale = downplayed and 0.65 or 1,
    glowBorder = false,
    glowXOffset = 0,
    glowYOffset = 0,
  }
end

-- Condition: when trigger 1 (the "missing" trigger) is Active, promote the glow from
-- its downplayed default to the full, attention-grabbing look. Used by Rend so the
-- glow is subtle during the final-tick countdown and full once the debuff drops.
-- (sub.3 = the glow sub-region: [1] subbackground, [2] subtext, [3] subglow.)
local function fullWhenMissingConditions()
  return {
    {
      check = { trigger = 1, variable = "show", value = 1 },
      changes = {
        { property = "sub.3.useGlowColor", value = false },
        { property = "sub.3.glowColor", value = { 1, 1, 1, 1 } },
        { property = "sub.3.glowScale", value = 1 },
      },
    },
  }
end

-- Build an `icon` aura with the two-trigger missing/countdown behavior plus glow.
local function iconAura(o, tocversion)
  local downplayed = (o.glow == "fullWhenOff")
  return {
    id = o.id,
    uid = o.uid,
    internalVersion = INTERNAL_VERSION,
    regionType = "icon",
    -- icon region defaults (RegionTypes/Icon.lua)
    icon = true,
    desaturate = false,
    iconSource = -1,
    inverse = false,
    width = 40,
    height = 40,
    color = { 1, 1, 1, 1 },
    alpha = 1.0,
    selfPoint = "CENTER",
    anchorPoint = "CENTER",
    anchorFrameType = "SCREEN",
    xOffset = o.x or 0,
    yOffset = o.y or 0,
    zoom = 0,
    keepAspectRatio = false,
    frameStrata = 1,
    cooldown = true,             -- swipe animates the remaining time on trigger 2
    cooldownTextDisabled = true, -- ...but the %p sub-text below is our countdown number
    cooldownSwipe = true,
    cooldownEdge = false,
    useCooldownModRate = true,
    triggers = {
      { trigger = auraTrigger(o, "showOnMissing"), untrigger = {} },
      {
        trigger = auraTrigger(o, "showOnActive", {
          useRem = true,
          remOperator = "<",
          rem = o.remThreshold, -- seconds left at which the countdown appears
        }),
        untrigger = {},
      },
      activeTriggerMode = -10, -- Private.trigger_modes.first_active
      disjunctive = "any",     -- show if EITHER trigger is active (missing OR expiring)
    },
    -- A centered countdown number; %p renders the remaining time of the active
    -- trigger, and is blank for the "missing" state (its duration is 0). Plus the
    -- Autocast Shine glow.
    subRegions = {
      { type = "subbackground" },
      {
        type = "subtext",
        text_text = "%p",
        text_color = { 1, 1, 1, 1 },
        text_font = DEFAULT_FONT,
        text_fontSize = 18,
        text_fontType = "OUTLINE",
        text_visible = true,
        text_justify = "CENTER",
        text_selfPoint = "AUTO",
        anchor_point = "CENTER",
        anchorXOffset = 0,
        anchorYOffset = 0,
        text_shadowColor = { 0, 0, 0, 1 },
        text_shadowXOffset = 0,
        text_shadowYOffset = 0,
        rotateText = "NONE",
        text_automaticWidth = "Auto",
        text_fixedWidth = 64,
        text_wordWrap = "WordWrap",
      },
      glowSubRegion(downplayed),
    },
    -- Rend downplays the glow by default (the countdown look) and promotes it to full
    -- when the debuff is off; Battle Shout glows full whenever shown, so no condition.
    conditions = downplayed and fullWhenMissingConditions() or {},
    load = {
      class = { multi = { WARRIOR = true } },
      use_class = true,
      use_combat = o.inCombat or nil, -- tristate: true => only while in combat
      size = { multi = {} },
      spec = { multi = {} },
      talent = { multi = {} },
    },
    actions = { init = {}, start = {}, finish = {} },
    animation = {
      start  = { type = "none", duration_type = "seconds", easeType = "none", easeStrength = 3 },
      main   = { type = "none", duration_type = "seconds", easeType = "none", easeStrength = 3 },
      finish = { type = "none", duration_type = "seconds", easeType = "none", easeStrength = 3 },
    },
    config = {},
    authorOptions = {},
    information = {},
    tocversion = tocversion,
  }
end

local function encode(data)
  local transmit = { m = "d", d = data, v = TRANSMIT_VERSION, s = WA_VERSION }
  local serialized = LibSerialize:SerializeEx({ errorOnUnserializableType = false }, transmit)
  local compressed = LibDeflate:CompressDeflate(serialized, { level = 9 })
  return "!WA:2!" .. LibDeflate:EncodeForPrint(compressed)
end

-- Decode a string we just produced and assert the critical fields survived.
local function verify(str, spec, tocversion)
  local compressed = LibDeflate:DecodeForPrint((str:gsub("^!WA:2!", "")))
  local serialized = LibDeflate:DecompressDeflate(compressed)
  local ok, t = LibSerialize:Deserialize(serialized)
  assert(ok and t and t.d, "round-trip decode failed for " .. spec.id)
  local d = t.d
  assert(d.regionType == "icon", "regionType")
  assert(d.tocversion == tocversion, "tocversion")
  assert(d.triggers.disjunctive == "any", "disjunctive")
  local t1, t2 = d.triggers[1].trigger, d.triggers[2].trigger
  assert(t1.type == "aura2" and t1.matchesShowOn == "showOnMissing", "trigger 1 (missing)")
  assert(t2.matchesShowOn == "showOnActive" and t2.useRem == true, "trigger 2 (active)")
  assert(tonumber(t2.rem) == spec.remThreshold and t2.remOperator == "<", "rem threshold")
  assert(t1.unit == spec.unit and t1.debuffType == spec.debuffType, "unit/debuffType")
  assert(t1.auranames[1] == spec.auraname, "auraname")
  assert(d.subRegions[2].type == "subtext" and d.subRegions[2].text_text == "%p", "countdown subtext")
  assert(d.subRegions[3].type == "subglow" and d.subRegions[3].glowType == "ACShine", "autocast glow")
  if spec.glow == "fullWhenOff" then
    assert(d.conditions[1].check.trigger == 1 and d.conditions[1].check.variable == "show",
      "glow condition checks trigger 1 active")
  else
    assert(#d.conditions == 0, "no conditions expected")
  end
end

local specs = {
  {
    id = "Battle Shout (Fiddlejig)",
    uid = "FjWarBShout0001",
    doc = "weakauras/warrior/battle-shout.md",
    unit = "player",
    debuffType = "HELPFUL",
    auraname = "Battle Shout",
    inCombat = nil,    -- always loaded, so you can re-buff before a pull
    remThreshold = 10, -- countdown shows in the last 10 seconds
    glow = "full",     -- Autocast Shine glows full whenever the icon is shown
    x = 0, y = -120,
  },
  {
    id = "Rend (Fiddlejig)",
    uid = "FjWarRend000001",
    doc = "weakauras/warrior/rend.md",
    unit = "target",
    debuffType = "HARMFUL",
    auraname = "Rend",
    inCombat = true,       -- only nag mid-fight, not when chatting up friendly NPCs
    remThreshold = 3,      -- countdown shows in the last ~tick (3s)
    glow = "fullWhenOff",  -- downplayed glow while counting down, full once it drops
    x = 0, y = -168,
  },
}

-- Replace the !WA:2! line under the flavor's "### <heading>" section of a doc.
local function writeDoc(spec, heading, str)
  local path = ROOT .. "/" .. spec.doc
  local f = assert(io.open(path, "r"), "doc not found: " .. path)
  local content = f:read("*a"); f:close()
  local hpos = content:find("### " .. heading:gsub("[%(%)%.%%%+%-%*%?%[%]%^%$]", "%%%0"), 1)
  assert(hpos, spec.doc .. " is missing a '### " .. heading .. "' section")
  local s, e = content:find("!WA:2![^\n]*", hpos)
  assert(s, spec.doc .. ": no !WA:2! line after '### " .. heading .. "'")
  local updated = content:sub(1, s - 1) .. str .. content:sub(e + 1)
  if updated ~= content then
    local w = assert(io.open(path, "w")); w:write(updated); w:close()
    return "updated"
  end
  return "unchanged"
end

local write = (arg and arg[1] == "--write")
for _, spec in ipairs(specs) do
  for _, flavor in ipairs(FLAVORS) do
    local str = encode(iconAura(spec, flavor.tocversion))
    verify(str, spec, flavor.tocversion)
    print(("%s — %s [toc %d]"):format(spec.id, flavor.heading, flavor.tocversion))
    if write then
      print("  doc: " .. writeDoc(spec, flavor.heading, str))
    else
      print(str)
      print()
    end
  end
end
