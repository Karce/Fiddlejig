-- Core.lua — Joja Mart's Auto-Petter
--
-- Picks the best food in your bags for the pet's diet and arms a secure button
-- so one keypress (or /click JojaAutoPetterButton) feeds it. Feeding a dynamically
-- chosen bag slot is a protected action, so it must go through a
-- SecureActionButtonTemplate whose attributes we set out of combat — which is why
-- this is an addon and not a macro.

local ADDON, ns = ...

local FEED_PET_SPELL_ID = 6991
local FEED_BUFF_SPELL_ID = 1539 -- "Feed Pet Effect": on the pet while it's eating

-- Happiness per feeding pulse is set by the level gap (petLevel - foodLevel):
--   gap <= 10        -> 35/pulse  (tier 3, "loves")
--   gap <= 20        -> 17/pulse  (tier 2, "likes")
--   gap <  30        -> 8/pulse   (tier 1, "eats")
--   gap >= 30        -> refused
local DELTA_LOVES, DELTA_LIKES, DELTA_EATS = 10, 20, 30

-- Remind when GetPetHappiness() drops below this (3 = no longer fully happy, so
-- you're losing the damage/loyalty bonus). Set to 2 to only nag when unhappy.
local REMINDER_BELOW = 3

-- Container API: globals on 2.5.x, C_Container on newer clients. Support both.
local C = _G.C_Container
local function NumSlots(bag)
	return (C and C.GetContainerNumSlots or GetContainerNumSlots)(bag)
end
local function SlotInfo(bag, slot)
	if C and C.GetContainerItemInfo then
		local info = C.GetContainerItemInfo(bag, slot)
		if info then return info.itemID, info.stackCount end
	else
		local _, count, _, _, _, _, _, _, _, itemID = GetContainerItemInfo(bag, slot)
		return itemID, count
	end
end

local feedButton
local rearmQueued = false
local lastRemindHappiness
local currentFood -- { bag, slot } currently armed, or nil

local function Print(msg)
	DEFAULT_CHAT_FRAME:AddMessage("|cff66bb88Auto-Petter|r " .. msg)
end

-- A summoned pet implies a Hunter with Feed Pet (both unlock at level 10), so the
-- localized spell name is enough — no need to gate on IsSpellKnown (which has been
-- unreliable for some Classic spells).
local function FeedPetSpellName()
	return (GetSpellInfo(FEED_PET_SPELL_ID))
end

-- Set of diet strings the current pet eats, or nil if no pet / unknown.
local function PetDiets()
	local list = { GetPetFoodTypes() }
	if #list == 0 then return nil end
	local set = {}
	for _, diet in ipairs(list) do set[diet] = true end
	return set
end

local function TierForDelta(delta)
	if delta <= DELTA_LOVES then return 3
	elseif delta <= DELTA_LIKES then return 2
	elseif delta < DELTA_EATS then return 1 end
	return 0
end

-- Best = highest happiness tier, then lowest food level (conserve better food),
-- then smallest stack (use up partial stacks first).
local function FindBestFood()
	if not UnitExists("pet") then return nil end
	local diets = PetDiets()
	if not diets then return nil end
	local petLevel = UnitLevel("pet")
	local best
	for bag = 0, 4 do
		for slot = 1, (NumSlots(bag) or 0) do
			local itemID, count = SlotInfo(bag, slot)
			local diet = itemID and ns.Foods[itemID]
			if diet and diets[diet] and not ns.Exclude[itemID] then
				local _, _, _, itemLevel = GetItemInfo(itemID)
				if itemLevel then
					local tier = TierForDelta(petLevel - itemLevel)
					if tier > 0 then
						local cand = { bag = bag, slot = slot, level = itemLevel, count = count or 1, tier = tier }
						if not best
							or cand.tier > best.tier
							or (cand.tier == best.tier and cand.level < best.level)
							or (cand.tier == best.tier and cand.level == best.level and cand.count < best.count) then
							best = cand
						end
					end
				end
			end
		end
	end
	return best
end

-- Arm the secure button with a feed macro: cast Feed Pet, then use the chosen bag
-- slot (the standard feed-macro form). type1 = the LeftButton action, matching the
-- "CLICK JojaAutoPetterButton:LeftButton" key binding.
local function ApplyFood(best)
	local spell = FeedPetSpellName()
	if best and spell then
		feedButton:SetAttribute("type1", "macro")
		feedButton:SetAttribute("macrotext", "/cast " .. spell .. "\n/use " .. best.bag .. " " .. best.slot)
		currentFood = best
	else
		feedButton:SetAttribute("type1", nil)
		feedButton:SetAttribute("macrotext", nil)
		currentFood = nil
	end
end

-- True while the pet is mid-meal (has the Feed Pet Effect buff). spellId is the
-- 10th return of UnitBuff on 2.5.x.
local function PetIsEating()
	if not UnitExists("pet") then return false end
	for i = 1, 40 do
		local name, _, _, _, _, _, _, _, _, spellId = UnitBuff("pet", i)
		if not name then break end
		if spellId == FEED_BUFF_SPELL_ID then return true end
	end
	return false
end

-- Re-pick and re-arm. Disarm (so a keypress does nothing) when there's no point
-- feeding: pet already Happy, or still eating the last meal. Secure attributes
-- can't change in combat, so defer if locked.
local function Rearm()
	if InCombatLockdown() then
		rearmQueued = true
		return
	end
	rearmQueued = false
	local best
	if UnitExists("pet") and not PetIsEating() then
		local happiness = GetPetHappiness()
		if not happiness or happiness < 3 then -- don't feed an already-Happy pet
			best = FindBestFood()
		end
	end
	ApplyFood(best)
end

local function CurrentFoodLink()
	if not currentFood then return nil end
	local getLink = (C and C.GetContainerItemLink) or GetContainerItemLink
	return getLink(currentFood.bag, currentFood.slot)
end

local function CheckHappiness()
	if not (JojaAutoPetterDB and JojaAutoPetterDB.reminder) then return end
	if InCombatLockdown() or not UnitExists("pet") then return end
	local happiness = GetPetHappiness()
	if not happiness then return end
	if happiness >= REMINDER_BELOW then
		lastRemindHappiness = nil
		return
	end
	if lastRemindHappiness == happiness then return end -- one nudge per state change
	lastRemindHappiness = happiness
	local link = CurrentFoodLink()
	if link then
		Print((UnitName("pet") or "Your pet") .. " is getting hungry — feed " .. link .. ".")
	else
		Print((UnitName("pet") or "Your pet") .. " is getting hungry — no suitable food found in bags.")
	end
	if SOUNDKIT then PlaySound(SOUNDKIT.TELL_MESSAGE) end
end

local function OnEvent(_, event, arg1)
	if event == "ADDON_LOADED" then
		if arg1 ~= ADDON then return end
		JojaAutoPetterDB = JojaAutoPetterDB or {}
		if JojaAutoPetterDB.reminder == nil then JojaAutoPetterDB.reminder = true end
	elseif event == "PLAYER_REGEN_ENABLED" then
		if rearmQueued then Rearm() end
		CheckHappiness()
	elseif event == "UNIT_HAPPINESS" then
		Rearm() -- happiness changed: (dis)arm per the not-already-Happy gate
		CheckHappiness()
	elseif event == "UNIT_AURA" then
		Rearm() -- Feed Pet Effect applied/faded: (dis)arm per the not-eating gate
	elseif event == "UNIT_PET" then
		lastRemindHappiness = nil
		Rearm()
	else -- PLAYER_LOGIN, PLAYER_ENTERING_WORLD, BAG_UPDATE_DELAYED
		Rearm()
	end
end

-- Dump full state to chat so we can see exactly where feeding breaks down.
local function Debug()
	Print("--- debug ---")
	Print("Feed Pet: name=" .. tostring(FeedPetSpellName()) ..
		"  IsSpellKnown=" .. tostring(IsSpellKnown(FEED_PET_SPELL_ID)))
	Print("pet: exists=" .. tostring(UnitExists("pet")) ..
		"  name=" .. tostring(UnitName("pet")) .. "  level=" .. tostring(UnitLevel("pet")))
	Print("gates: happiness=" .. tostring(GetPetHappiness()) .. " (3=Happy → won't arm)" ..
		"  eating=" .. tostring(PetIsEating()) .. " (true → won't arm)")
	local diets = { GetPetFoodTypes() }
	Print("diet types (" .. #diets .. "): [" .. table.concat(diets, ", ") .. "]")
	Print("container API: " .. ((C and C.GetContainerItemInfo) and "C_Container" or "global"))
	Print("combat lockdown: " .. tostring(InCombatLockdown()))
	local dietset = {}
	for _, d in ipairs(diets) do dietset[d] = true end
	local scanned, matched, shown = 0, 0, 0
	for bag = 0, 4 do
		for slot = 1, (NumSlots(bag) or 0) do
			local itemID = SlotInfo(bag, slot)
			if itemID then
				scanned = scanned + 1
				local diet = ns.Foods[itemID]
				if diet and dietset[diet] then
					matched = matched + 1
					if shown < 6 then
						shown = shown + 1
						local name, _, _, ilvl = GetItemInfo(itemID)
						Print(string.format("  match: %s (id %d, ilvl %s, %s)", tostring(name), itemID, tostring(ilvl), diet))
					end
				end
			end
		end
	end
	Print("scanned " .. scanned .. " items; " .. matched .. " match the pet's diet")
	local best = FindBestFood()
	if best then
		local getLink = (C and C.GetContainerItemLink) or GetContainerItemLink
		Print(string.format("best: %s (bag %d slot %d, level %d, tier %d)",
			tostring(getLink(best.bag, best.slot)), best.bag, best.slot, best.level, best.tier))
	else
		Print("best: NONE selected")
	end
	Print("armed: type1=" .. tostring(feedButton:GetAttribute("type1")) ..
		"  macrotext=" .. tostring(feedButton:GetAttribute("macrotext")))
	Print("bound key for \"Feed Pet (best food)\": " ..
		tostring(GetBindingKey("CLICK JojaAutoPetterButton:LeftButton")))
end

local function SlashHandler(msg)
	msg = (msg or ""):lower():gsub("%s+", "")
	if msg == "on" or msg == "reminderon" then
		JojaAutoPetterDB.reminder = true
		Print("hunger reminder: on")
	elseif msg == "off" or msg == "reminderoff" then
		JojaAutoPetterDB.reminder = false
		Print("hunger reminder: off")
	elseif msg == "debug" then
		Debug()
	elseif msg == "feed" then
		feedButton:Click()
	else
		if not UnitExists("pet") then
			Print("No pet out — summon or revive your pet first.")
			return
		end
		local link = CurrentFoodLink()
		Print("best food: " .. (link or "none found") ..
			"  |  reminder: " .. (JojaAutoPetterDB.reminder and "on" or "off"))
		Print("Feed with your bound key, /joja feed, or a /click JojaAutoPetterButton macro. /joja debug to diagnose.")
	end
end

feedButton = CreateFrame("Button", "JojaAutoPetterButton", UIParent, "SecureActionButtonTemplate")
-- This client's "CLICK ...:LeftButton" binding fires a key-DOWN click, so register
-- that edge. Single edge = one feed per press; the buff/Happy gates stop the rest.
feedButton:RegisterForClicks("AnyDown")
-- Present (so /click and key bindings work) but out of sight; you trigger it by key/macro.
feedButton:SetSize(1, 1)
feedButton:SetPoint("BOTTOMLEFT", -100, -100)
feedButton:SetAlpha(0)

local driver = CreateFrame("Frame")
driver:SetScript("OnEvent", OnEvent)
driver:RegisterEvent("ADDON_LOADED")
driver:RegisterEvent("PLAYER_LOGIN")
driver:RegisterEvent("PLAYER_ENTERING_WORLD")
driver:RegisterEvent("BAG_UPDATE_DELAYED")
driver:RegisterEvent("UNIT_HAPPINESS")
driver:RegisterEvent("PLAYER_REGEN_ENABLED")
driver:RegisterUnitEvent("UNIT_PET", "player")
driver:RegisterUnitEvent("UNIT_AURA", "pet") -- catch Feed Pet Effect applying/fading

SLASH_JOJAAUTOPETTER1 = "/joja"
SLASH_JOJAAUTOPETTER2 = "/autopetter"
SlashCmdList.JOJAAUTOPETTER = SlashHandler

-- Key Bindings entry. The "CLICK <button>:LeftButton" action is the canonical secure
-- way to bind a key to a SecureActionButton (same as RXPGuides' item buttons); the
-- category names the section via the BINDING_HEADER_* global it points at.
_G.BINDING_HEADER_JojaAutoPetter = "Joja Mart's Auto-Petter"
_G["BINDING_NAME_CLICK JojaAutoPetterButton:LeftButton"] = "Feed Pet (best food)"
