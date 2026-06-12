-- Bar.lua — ShotClock
--
-- Draws the Auto Shot cycle tracked by Core.lua as a single bar:
--   reload phase — green fill shrinking toward empty as the reload completes;
--   aim phase    — red fill growing over the ~0.5s aim window; it visibly snaps
--                  back if you move, because the engine restarts the aim.
-- Visible whenever you're in combat or shooting (always while unlocked).

local ADDON, ns = ...

local FONT = "Fonts/FRIZQT__.ttf"
local FONT_SIZE = 12
local BAR_TEXTURE = "Interface/AddOns/ShotClock/Images/Bar"
local BACKGROUND_TEXTURE = "Interface/AddOns/ShotClock/Images/Background"

local frame

local function OnDragStart()
	if not ns.db.locked then
		frame:StartMoving()
	end
end

local function OnDragStop()
	frame:StopMovingOrSizing()
	local point, _, relPoint, x, y = frame:GetPoint()
	if x < 20 and x > -20 then x = 0 end -- snap to horizontal center
	local db = ns.db
	db.point = point
	db.relPoint = relPoint
	db.x = ns.Round(x, 1)
	db.y = ns.Round(y, 1)
	ns.ApplyLayout()
end

function ns.CreateBar()
	frame = CreateFrame("Frame", "ShotClockFrame", UIParent)
	frame:SetMovable(true)
	frame:RegisterForDrag("LeftButton")
	frame:SetScript("OnDragStart", OnDragStart)
	frame:SetScript("OnDragStop", OnDragStop)

	frame.backplane = CreateFrame("Frame", nil, frame, "BackdropTemplate")
	frame.backplane:SetPoint("TOPLEFT", -9, 9)
	frame.backplane:SetPoint("BOTTOMRIGHT", 9, -9)
	frame.backplane:SetFrameStrata("BACKGROUND")
	frame.backplane:SetBackdrop({
		bgFile = BACKGROUND_TEXTURE,
		tile = true, tileSize = 16,
		insets = { left = 8, right = 8, top = 8, bottom = 8 },
	})
	frame.backplane:SetBackdropColor(0, 0, 0, 0.5)

	frame.bar = frame:CreateTexture(nil, "ARTWORK")
	frame.bar:SetTexture(BAR_TEXTURE)
	frame.bar:SetPoint("BOTTOM", 0, 0)

	frame.text = frame:CreateFontString(nil, "OVERLAY")
	frame.text:SetFont(FONT, FONT_SIZE)
	frame.text:SetTextColor(1, 1, 1, 1)
	frame.text:SetJustifyV("MIDDLE")
	frame.text:SetJustifyH("CENTER")

	frame:Show()
end

function ns.ApplyLayout()
	local db = ns.db
	frame:EnableMouse(not db.locked)
	frame:SetScale(db.scale)
	frame:ClearAllPoints()
	frame:SetPoint(db.point, UIParent, db.relPoint, db.x, db.y)
	frame:SetSize(db.width, db.height)
	frame.bar:SetHeight(db.height)
	frame.text:SetPoint("BOTTOMRIGHT", -5, (db.height / 2) - (FONT_SIZE / 2))
	if db.showText then
		frame.text:Show()
	else
		frame.text:Hide()
	end
end

function ns.UpdateBar()
	local db = ns.db
	local state = ns.state

	frame.text:SetText(tostring(ns.Round(state.shot_timer, 0.1)))

	-- Unlocked overrides the fade so the bar can be dragged out of combat.
	if state.in_combat or state.shooting or not db.locked then
		frame:SetAlpha(1)
	else
		frame:SetAlpha(0)
	end

	local width
	if state.auto_shot_ready then
		frame.bar:SetVertexColor(0.8, 0, 0, 1)
		width = db.width * (state.auto_cast_time - state.shot_timer) / state.auto_cast_time
	else
		frame.bar:SetVertexColor(0.2, 0.8, 0.2, 1)
		width = db.width * (state.shot_timer - state.auto_cast_time)
			/ (state.range_speed - state.auto_cast_time)
	end
	if width < 2 then width = 2 end
	frame.bar:SetWidth(math.min(width, db.width))
end
