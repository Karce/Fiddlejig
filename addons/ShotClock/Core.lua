-- Core.lua — ShotClock
--
-- Hunter Auto Shot timer engine. Each Auto Shot cycle is a reload (weapon speed
-- minus ~0.5s) followed by a ~0.5s "aim" window: moving or casting during the aim
-- restarts it, delaying your own shot. This file tracks that cycle; Bar.lua draws it.
--
-- The timer model is extracted from WeaponSwingTimer by LeftHandedGlove, as
-- maintained in WeaponSwingTimer-SixxFix by WatchYourSixx
-- (github.com/watchyoursixx/WeaponSwingTimer-SixxFix). See README.md.

local ADDON, ns = ...

local AUTO_SHOT = 75
local FEIGN_DEATH = 5384
-- Aimed Shot ranks 1-6 (Era) + 7 (TBC). A successful Aimed Shot restarts the cycle.
local AIMED_SHOT_IDS = {
	[19434] = true, [20900] = true, [20901] = true, [20902] = true,
	[20903] = true, [20904] = true, [27065] = true,
}
-- Trueshot Aura ranks 1-3 (Era) + 4 (TBC; missing upstream). Casting it resets the
-- cycle to the unhasted weapon speed, same as Feign Death.
local TRUESHOT_IDS = { [19506] = true, [20905] = true, [20906] = true, [27066] = true }

local DEFAULTS = {
	width = 300,
	height = 12,
	scale = 1.0,
	locked = false,
	showText = true,
	point = "CENTER", relPoint = "CENTER", x = 0, y = -260,
}

-- Recent Era builds shim GetSpellInfo away in favor of C_Spell; support both.
local GetSpellInfoCompat = GetSpellInfo or function(spellID)
	local info = C_Spell.GetSpellInfo(spellID)
	if info then return info.name, nil, info.iconID, info.castTime end
end

local function Print(msg)
	DEFAULT_CHAT_FRAME:AddMessage("|cffd4a017ShotClock|r " .. msg)
end

function ns.Round(num, step)
	return math.floor(num / step + 0.5) * step
end

-- Shared timer state, read every frame by Bar.lua.
local state = {
	shooting = false,        -- Auto Shot toggled on (START/STOP_AUTOREPEAT_SPELL)
	casting = false,         -- a cast-time spell (e.g. Aimed Shot) is in progress
	in_combat = false,
	has_moved = false,
	range_speed = 3,         -- current (hasted) ranged weapon speed
	base_speed = 1,          -- unhasted speed from the weapon tooltip
	haste_modifier = 1,      -- range_speed / base_speed
	auto_cast_time = 0.52,   -- aim-window length, scaled by haste each frame
	shot_timer = 0.52,       -- the countdown driving the bar
	last_shot_time = 0,
	auto_shot_ready = true,  -- false = reload phase, true = aim phase
	feign_armed = false,     -- Feign Death active; breaking it resets the cycle
	feign_reset_done = false,
}
ns.state = state

-- Unhasted weapon speed isn't exposed by the API (UnitRangedDamage is post-haste),
-- so parse the "Speed X.XX" line off the equipped ranged weapon's tooltip.
local speedTooltip
local speedCache = {}
local SPEED_PATTERN = SPEED .. " (%d+%.%d%d)"

local function GetRangedBaseSpeed()
	local weaponID = GetInventoryItemID("player", INVSLOT_RANGED)
	if not weaponID then return 1 end
	if speedCache[weaponID] then return speedCache[weaponID] end

	local speed = 1
	speedTooltip:ClearLines()
	speedTooltip:SetItemByID(weaponID)
	for i = 1, speedTooltip:NumLines() do
		local line = _G["ShotClockTooltipTextRight" .. i]
		local text = line and line:GetText()
		local match = text and text:match(SPEED_PATTERN)
		if match then
			speed = tonumber(match)
			break
		end
	end
	speedCache[weaponID] = speed
	return speed
end

local function UpdateHasteModifier()
	if state.base_speed == 1 then
		-- Tooltip not parsed yet (or no weapon); keep retrying until it is.
		state.base_speed = GetRangedBaseSpeed()
	else
		local speed = UnitRangedDamage("player")
		if not speed or speed == 0 then
			state.haste_modifier = 1
		else
			state.haste_modifier = speed / state.base_speed
		end
	end
end

local function ResetShotTimer()
	local now = GetTime()
	if (now + 0.05 - state.last_shot_time) > (state.range_speed - state.auto_cast_time) then
		-- Past the reload (e.g. aim restarted by movement): back to a full aim window.
		state.shot_timer = state.auto_cast_time
		state.auto_shot_ready = true
	elseif now ~= state.last_shot_time and not state.casting then
		state.shot_timer = now - state.last_shot_time
		state.auto_shot_ready = false
	elseif state.casting then
		-- Bail out of a stuck cast after a hasted 3s (longest cast: Aimed Shot).
		if (now - state.last_shot_time) > (3 * state.haste_modifier) then
			state.shot_timer = state.auto_cast_time
		end
	else
		state.shot_timer = state.range_speed
		state.auto_shot_ready = false
	end
end

-- Feign Death / Trueshot Aura restart the cycle at the *unhasted* weapon speed
-- (+0.15s observed penalty), once per feign.
local function ResetFromFeign()
	state.last_shot_time = GetTime()
	if not state.feign_reset_done then
		state.range_speed = GetRangedBaseSpeed() + 0.15
		state.feign_reset_done = true
	end
	ResetShotTimer()
end

local function UpdateTimer(elapsed)
	if state.shot_timer < 0 then
		state.shot_timer = 0
	else
		state.shot_timer = state.shot_timer - elapsed
	end
	UpdateHasteModifier()
	-- Upstream's model scales the 0.5s aim window with haste; kept as-is.
	state.auto_cast_time = 0.52 * state.haste_modifier

	-- Moving or casting during the aim window restarts the aim.
	if (state.has_moved or state.casting) and state.shot_timer <= state.auto_cast_time then
		ResetShotTimer()
	end
	if state.shot_timer <= state.auto_cast_time then
		state.auto_shot_ready = true
		if not state.shooting then
			ResetShotTimer()
		end
	else
		state.auto_shot_ready = false
	end
end

local function OnUpdate(_, elapsed)
	state.has_moved = GetUnitSpeed("player") > 0
	if state.feign_armed and state.has_moved then
		ResetFromFeign()
		state.feign_armed = false
	end
	UpdateTimer(elapsed)
	ns.UpdateBar()
end

-- Any cast-time spell (not Auto Shot itself) pauses the cycle while it casts.
local function OnSpellCastStart(spellID)
	if state.casting or not UnitCanAttack("player", "target") then return end
	local _, _, _, castTime = GetSpellInfoCompat(spellID)
	if castTime and castTime > 0 and spellID ~= AUTO_SHOT then
		state.casting = true
	end
end

local function OnCombatLog()
	local _, event, _, sourceGUID, _, _, _, _, _, _, _, spellID = CombatLogGetCurrentEventInfo()
	if event == "SPELL_CAST_START" and sourceGUID == UnitGUID("player") then
		state.feign_armed = false
		OnSpellCastStart(spellID)
	end
end

-- UNIT_SPELLCAST_SUCCEEDED (player-filtered): the shot landed — restart the cycle,
-- and re-read the hasted speed so in-combat haste buffs take effect next shot.
local function OnSpellSucceeded(spellID)
	state.casting = false
	if spellID == FEIGN_DEATH or TRUESHOT_IDS[spellID] then
		if spellID == FEIGN_DEATH then
			state.feign_armed = true
		end
		ResetFromFeign()
		return
	end
	if AIMED_SHOT_IDS[spellID] then
		state.feign_reset_done = false
		state.last_shot_time = GetTime()
		ResetShotTimer()
	end
	if spellID == AUTO_SHOT then
		state.feign_reset_done = false
		state.last_shot_time = GetTime()
		ResetShotTimer()
		local newSpeed = UnitRangedDamage("player")
		if not newSpeed or newSpeed == 0 then
			newSpeed = state.range_speed
		end
		if newSpeed ~= state.range_speed then
			if not state.auto_shot_ready then
				-- Mid-reload haste change: rescale the remaining time proportionally.
				state.shot_timer = state.shot_timer * (newSpeed / state.range_speed)
			end
			state.range_speed = newSpeed
		end
	end
end

-- A failed Auto Shot attempt (target too close / not facing) makes the server
-- retry every 0.5s — reflect that delay on the bar.
local function OnAutoShotFailedQuiet(spellID)
	if spellID ~= AUTO_SHOT then return end
	if not state.casting and state.shooting
		and (GetTime() - state.last_shot_time) > (state.range_speed - state.auto_cast_time) then
		state.shot_timer = state.auto_cast_time + 0.5
	end
end

local function Status()
	local db = ns.db
	Print(string.format("weapon speed %.2f (base %.2f) | %dx%d, scale %.1f, %s, text %s",
		state.range_speed, state.base_speed, db.width, db.height, db.scale,
		db.locked and "locked" or "unlocked", db.showText and "on" or "off"))
	Print("commands: lock | unlock | width <px> | height <px> | scale <0.5-2> | text on/off | reset")
end

local function SlashHandler(msg)
	local db = ns.db
	local cmd, arg = msg:lower():match("^(%S*)%s*(.-)%s*$")
	if cmd == "lock" then
		db.locked = true
		Print("bar locked")
	elseif cmd == "unlock" then
		db.locked = false
		Print("bar unlocked — drag to move, /sc lock when done")
	elseif cmd == "width" or cmd == "height" then
		local n = tonumber(arg)
		if n then
			db[cmd] = math.max(cmd == "width" and 50 or 4, math.min(n, 600))
		else
			Print("usage: /sc " .. cmd .. " <pixels>")
		end
	elseif cmd == "scale" then
		local n = tonumber(arg)
		if n then
			db.scale = math.max(0.5, math.min(n, 2))
		else
			Print("usage: /sc scale <0.5-2>")
		end
	elseif cmd == "text" then
		if arg == "on" or arg == "off" then
			db.showText = (arg == "on")
		else
			Print("usage: /sc text on|off")
		end
	elseif cmd == "reset" then
		for k, v in pairs(DEFAULTS) do db[k] = v end
		Print("settings reset")
	else
		Status()
		return
	end
	ns.ApplyLayout()
end

local driver = CreateFrame("Frame")

local function Init()
	ShotClockDB = ShotClockDB or {}
	for k, v in pairs(DEFAULTS) do
		if ShotClockDB[k] == nil then ShotClockDB[k] = v end
	end
	ns.db = ShotClockDB

	speedTooltip = CreateFrame("GameTooltip", "ShotClockTooltip", nil, "GameTooltipTemplate")
	speedTooltip:SetOwner(WorldFrame, "ANCHOR_NONE")
	state.base_speed = GetRangedBaseSpeed()
	state.last_shot_time = GetTime()
	state.in_combat = InCombatLockdown()

	ns.CreateBar()
	ns.ApplyLayout()

	driver:RegisterEvent("PLAYER_REGEN_ENABLED")
	driver:RegisterEvent("PLAYER_REGEN_DISABLED")
	driver:RegisterEvent("COMBAT_LOG_EVENT_UNFILTERED")
	driver:RegisterUnitEvent("UNIT_INVENTORY_CHANGED", "player")
	driver:RegisterEvent("START_AUTOREPEAT_SPELL")
	driver:RegisterEvent("STOP_AUTOREPEAT_SPELL")
	driver:RegisterUnitEvent("UNIT_SPELLCAST_SUCCEEDED", "player")
	driver:RegisterUnitEvent("UNIT_SPELLCAST_FAILED", "player")
	driver:RegisterUnitEvent("UNIT_SPELLCAST_INTERRUPTED", "player")
	-- Not guaranteed on every Classic build; losing it only costs the 0.5s
	-- failed-shot retry display.
	pcall(driver.RegisterUnitEvent, driver, "UNIT_SPELLCAST_FAILED_QUIET", "player")
	driver:SetScript("OnUpdate", OnUpdate)

	-- Jumping breaks Feign Death before GetUnitSpeed sees any movement.
	if type(JumpOrAscendStart) == "function" then
		hooksecurefunc("JumpOrAscendStart", function()
			if state.feign_armed then
				ResetFromFeign()
				state.feign_armed = false
			end
		end)
	end

	SLASH_SHOTCLOCK1 = "/shotclock"
	SLASH_SHOTCLOCK2 = "/sc"
	SlashCmdList.SHOTCLOCK = SlashHandler
end

driver:SetScript("OnEvent", function(_, event, ...)
	if event == "ADDON_LOADED" then
		if ... ~= ADDON then return end
		driver:UnregisterEvent("ADDON_LOADED")
		local _, class = UnitClass("player")
		if class == "HUNTER" then Init() end
	elseif event == "PLAYER_REGEN_ENABLED" then
		state.in_combat = false
	elseif event == "PLAYER_REGEN_DISABLED" then
		state.in_combat = true
	elseif event == "COMBAT_LOG_EVENT_UNFILTERED" then
		OnCombatLog()
	elseif event == "UNIT_INVENTORY_CHANGED" then
		state.base_speed = GetRangedBaseSpeed()
	elseif event == "START_AUTOREPEAT_SPELL" then
		state.shooting = true
	elseif event == "STOP_AUTOREPEAT_SPELL" then
		state.shooting = false
	elseif event == "UNIT_SPELLCAST_SUCCEEDED" then
		OnSpellSucceeded(select(3, ...))
	elseif event == "UNIT_SPELLCAST_FAILED" or event == "UNIT_SPELLCAST_INTERRUPTED" then
		-- Clears a stuck `casting` when a cast fails (target died, out of range).
		state.casting = false
	elseif event == "UNIT_SPELLCAST_FAILED_QUIET" then
		OnAutoShotFailedQuiet(select(3, ...))
	end
end)
driver:RegisterEvent("ADDON_LOADED")
