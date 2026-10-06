-- Dragon Ball: Sparking! ZERO - Complete Story Runtime Mod
-- Target: RE-UE4SS v3.0.1 Beta (UE 5.1.1)
--
-- Responsibilities:
-- 1. Intercept SSDragonAdventureIFCSManager:IsPlayable to display "New Game" instead of "Unlock".
-- 2. Intercept SSDragonAdventureIFCSManager:IsModeStart to authorize mode start and prevent the NEO store dialog.
-- 3. GameThread-safe execution without Slate/UMG widget reflection (ADR 0004).

local MOD_TAG = "[CompleteStory]"
local MANAGER_PATH = "/Script/SS.SSDragonAdventureIFCSManager:"
local MENU_PATH = "/Script/SS.SSBuiltInMenu:"

local sequence = 0

local function log_event(name, details)
    sequence = sequence + 1
    print(string.format("%s TRACE %04d %s%s\n", MOD_TAG, sequence, name,
        details and (" " .. details) or ""))
end

local function safe_name(value)
    if value == nil then return "<nil>" end
    local ok, inner = pcall(function() return value:get() end)
    local object = ok and inner or value
    if object == nil then return "<nil>" end
    local valid_ok, valid = pcall(function() return object:IsValid() end)
    if not valid_ok or not valid then return "<invalid>" end
    local name_ok, name = pcall(function() return object:GetFullName() end)
    if name_ok and name ~= nil then return tostring(name) end
    return "<unnamed>"
end

local function init_mod()
    -- Hook 1: IsPlayable -> Controls carousel button label (Unlock vs New Game)
    local ok_play, pre_play, post_play = pcall(function()
        return RegisterHook(
            MANAGER_PATH .. "IsPlayable",
            function(context)
                log_event("IS_PLAYABLE_PRE", "context=" .. safe_name(context))
                return nil
            end,
            function(context, return_value)
                log_event("IS_PLAYABLE_POST", "Overriding ReturnValue to true")
                pcall(function()
                    if return_value ~= nil and type(return_value.set) == "function" then
                        return_value:set(true)
                    end
                end)
                return true
            end
        )
    end)
    if not ok_play then
        log_event("HOOK_FAILED", "IsPlayable error=" .. tostring(pre_play))
    else
        log_event("HOOK_REGISTERED", string.format("IsPlayable pre=%s post=%s", tostring(pre_play), tostring(post_play)))
    end

    -- Hook 2: IsModeStart -> Controls authorization to start campaign (bypasses NEO store modal)
    local ok_start, pre_start, post_start = pcall(function()
        return RegisterHook(
            MANAGER_PATH .. "IsModeStart",
            function(context)
                log_event("IS_MODE_START_PRE", "context=" .. safe_name(context))
                return nil
            end,
            function(context, return_value)
                log_event("IS_MODE_START_POST", "Overriding ReturnValue to true")
                pcall(function()
                    if return_value ~= nil and type(return_value.set) == "function" then
                        return_value:set(true)
                    end
                end)
                return true
            end
        )
    end)
    if not ok_start then
        log_event("HOOK_FAILED", "IsModeStart error=" .. tostring(pre_start))
    else
        log_event("HOOK_REGISTERED", string.format("IsModeStart pre=%s post=%s", tostring(pre_start), tostring(post_start)))
    end

    -- Hook 3: Button decision logging
    pcall(function()
        RegisterHook(
            MENU_PATH .. "NewDecideButton",
            function(context, menu_button)
                log_event("NEW_DECIDE_BUTTON_PRE", string.format("context=%s button=%s",
                    safe_name(context), safe_name(menu_button)))
                return nil
            end
        )
    end)

    log_event("MOD_READY", "IsPlayable and IsModeStart active")
end

log_event("INITIALIZING", "version=v0.8-playable-modestart")
ExecuteInGameThread(init_mod)
