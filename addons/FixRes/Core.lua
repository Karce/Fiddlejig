-- Core.lua — FixRes
--
-- Auto-corrects broken startup resolution on some Linux compositors. Via
-- Proton the client can start with a work-area-clipped "Custom" resolution
-- despite Config.wtf holding correct fullscreen values. FixRes detects the
-- mismatch at login and re-applies the desired resolution once.

local ADDON, ns = ...

local driver = CreateFrame("Frame")
local fired = false

local function Print(msg)
	DEFAULT_CHAT_FRAME:AddMessage("|cff33ff99FixRes:|r " .. msg)
end

local function ParseRes(str)
	if not str then return nil, nil end
	local w, h = str:match("(%d+)x(%d+)")
	if w then return tonumber(w), tonumber(h) end
	return nil, nil
end

local function GetCVarSafe(name)
	local fn = (C_CVar and C_CVar.GetCVar) or GetCVar
	if not fn then return nil end
	local ok, val = pcall(fn, name)
	return ok and val or nil
end

local function SetCVarSafe(name, value)
	local fn = (C_CVar and C_CVar.SetCVar) or SetCVar
	if not fn then return false end
	local ok, result = pcall(fn, name, value)
	return ok and result
end

-- Detect actual game resolution via available APIs.
local function GetActualResolution()
	-- Legacy API: queries GX engine state directly, not CVar-backed.
	if GetCurrentResolution and GetScreenResolutions then
		local ok, idx = pcall(GetCurrentResolution)
		if ok and idx and idx > 0 then
			local ok2, all = pcall(function() return {GetScreenResolutions()} end)
			if ok2 and all and all[idx] then
				return ParseRes(all[idx])
			end
		end
	end
	-- gxResolution CVar: may reflect live GX state on some clients.
	return ParseRes(GetCVarSafe("gxResolution"))
end

-- Desired resolution: SavedVariables override > gxFullscreenResolution CVar > default.
local function GetDesiredResolution()
	if FixResDB and FixResDB.width and FixResDB.height then
		return FixResDB.width, FixResDB.height
	end
	local w, h = ParseRes(GetCVarSafe("gxFullscreenResolution"))
	if w then return w, h end
	return 2560, 1440
end

local function ApplyFix(dw, dh)
	if not RestartGx then
		Print("RestartGx unavailable")
		return
	end
	local res = dw .. "x" .. dh
	SetCVarSafe("gxFullscreenResolution", res)
	SetCVarSafe("gxMaximize", "1")
	pcall(RestartGx)
	Print("corrected resolution to " .. res)
	if C_Timer and C_Timer.After then
		C_Timer.After(2, function()
			local aw, ah = GetActualResolution()
			if aw and (aw ~= dw or ah ~= dh) then
				Print("warning: still " .. aw .. "x" .. ah .. " — try manual Graphics settings")
			end
		end)
	end
end

local function CheckAndFix()
	local dw, dh = GetDesiredResolution()
	local aw, ah = GetActualResolution()
	if not aw then return end
	if aw == dw and ah == dh then return end
	ApplyFix(dw, dh)
end

driver:RegisterEvent("ADDON_LOADED")
driver:RegisterEvent("PLAYER_ENTERING_WORLD")
driver:SetScript("OnEvent", function(self, event, arg1)
	if event == "ADDON_LOADED" then
		if arg1 ~= ADDON then return end
		FixResDB = FixResDB or {}
		self:UnregisterEvent("ADDON_LOADED")
	elseif event == "PLAYER_ENTERING_WORLD" then
		if fired then return end
		-- Only on initial login; skip zone transitions and /reload.
		-- If isInitialLogin is nil (client not passing the arg), treat as initial.
		if arg1 == false then return end
		fired = true
		self:UnregisterEvent("PLAYER_ENTERING_WORLD")
		CheckAndFix()
	end
end)

SLASH_FIXRES1 = "/fixres"
SlashCmdList.FIXRES = function(msg)
	msg = strlower(strtrim(msg or ""))
	if msg == "status" then
		local dw, dh = GetDesiredResolution()
		local aw, ah = GetActualResolution()
		Print("desired: " .. dw .. "x" .. dh)
		Print("actual: " .. (aw and (aw .. "x" .. ah) or "detection unavailable"))
		if FixResDB.width then
			Print("override: " .. FixResDB.width .. "x" .. FixResDB.height)
		end
		return
	end
	if msg == "apply" then
		ApplyFix(GetDesiredResolution())
		return
	end
	local w, h = msg:match("(%d+)x(%d+)")
	if w then
		FixResDB.width = tonumber(w)
		FixResDB.height = tonumber(h)
		Print("override set to " .. w .. "x" .. h .. " (applies next login)")
		return
	end
	Print("/fixres <W>x<H> | status | apply")
end
