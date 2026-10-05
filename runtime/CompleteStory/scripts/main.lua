-- Dragon Ball: Sparking! ZERO - Complete Story Runtime Mod
-- Target: RE-UE4SS v3.0.1 Beta (UE 5.1.1)
--
-- Responsibilities:
-- 1. Safely intercept /Script/SS.SSDragonAdventureIFCSManager:IsPlayable on the GameThread.
-- 2. Query native manager properties via protected calls (pcall) without reflecting UI widgets (ADR 0004).
-- 3. Override ReturnValue to true strictly for Complete Story (route key 0000_00).
-- 4. Preserve vanilla behavior (nil return) for all 12 stock character campaigns.

local MOD_TAG = "[CompleteStory]"
local TARGET_ROUTE_KEY = "0000_00"
local HOOK_FUNCTION = "/Script/SS.SSDragonAdventureIFCSManager:IsPlayable"

local last_logged_key = nil
local hook_registered = false

local function log_line(message)
    print(string.format("%s %s\n", MOD_TAG, message))
end

local function unwrap(context)
    if context == nil then return nil end
    local ok, value = pcall(function() return context:get() end)
    if ok and value ~= nil then return value end
    return context
end

local function is_valid(object)
    if object == nil then return false end
    local ok, result = pcall(function() return object:IsValid() end)
    return ok and result == true
end

-- Safely inspect native C++ properties on SSDragonAdventureIFCSManager
-- Strictly BANNED per ADR 0004: FindAllOf("TextBlock"), UI panel reflection, and Slate scraping.
local function resolve_focused_route_key(manager)
    if not is_valid(manager) then return nil end

    -- Probe 1: Direct FName / Key struct property on manager if exposed
    local ok_key, key_prop = pcall(function() return manager.SelectCharacterKey end)
    if ok_key and key_prop ~= nil then
        local to_str_ok, key_str = pcall(function() return key_prop:ToString() end)
        if to_str_ok and key_str ~= nil and key_str ~= "" then
            return key_str
        end
    end

    -- Probe 2: CurrentCharacterData pointer -> Key resolution
    local ok_data, char_data = pcall(function() return manager.CurrentCharacterData end)
    if ok_data and is_valid(char_data) then
        local name_ok, data_name = pcall(function() return char_data:GetFullName() end)
        if name_ok and data_name ~= nil and string.find(data_name, "DAIF_CharaData_CompleteStory", 1, true) then
            return TARGET_ROUTE_KEY
        end
    end

    return nil
end

local function is_playable_post(context)
    local manager = unwrap(context)
    if not is_valid(manager) then return nil end

    local route_key = resolve_focused_route_key(manager)

    if route_key ~= last_logged_key then
        last_logged_key = route_key
        log_line(string.format("IsPlayable evaluated; resolved route_key=%s", route_key or "<native/vanilla>"))
    end

    -- Guarded override: unlock strictly Complete Story
    if route_key == TARGET_ROUTE_KEY then
        log_line("Overriding IsPlayable ReturnValue to true for Complete Story (0000_00)")
        return true
    end

    -- Return nil to let Bandai's native C++ save validation govern all 12 stock characters
    return nil
end

local function init_mod()
    if hook_registered then return end

    local ok, pre_id, post_id = pcall(function()
        return RegisterHook(
            HOOK_FUNCTION,
            function(context) return nil end,
            is_playable_post
        )
    end)

    if ok then
        hook_registered = true
        log_line(string.format("Registered native hook on %s (pre=%s, post=%s)", HOOK_FUNCTION, tostring(pre_id), tostring(post_id)))
    else
        log_line(string.format("FATAL: Failed to register hook on %s: %s", HOOK_FUNCTION, tostring(pre_id)))
    end
end

log_line("Initializing Complete Story runtime module (RE-UE4SS v3.0.1 Beta)")
ExecuteInGameThread(init_mod)
