-- Dragon Ball: Sparking! ZERO - Complete Story runtime trace
-- Target: RE-UE4SS v3.0.1 Beta (UE 5.1.1)
--
-- Gate 1 diagnostic: compare the stock and Complete Story native menu paths.
-- This script reads only hook context and parameters supplied by verified
-- UFunctions. It does not inspect Slate/UMG trees or alter native results.

local MOD_TAG = "[CompleteStory]"
local MANAGER_PATH = "/Script/SS.SSDragonAdventureIFCSManager:"
local MENU_PATH = "/Script/SS.SSBuiltInMenu:"

local sequence = 0
local hook_ids = {}

local function log_event(name, details)
    sequence = sequence + 1
    print(string.format("%s TRACE %04d %s%s\n", MOD_TAG, sequence, name,
        details and (" " .. details) or ""))
end

local function unwrap(value)
    if value == nil then return nil end
    local ok, inner = pcall(function() return value:get() end)
    if ok then return inner end
    return value
end

local function safe_bool(value)
    local inner = unwrap(value)
    if inner == true then return "true" end
    if inner == false then return "false" end
    return "<unavailable>"
end

local function safe_name(value)
    local object = unwrap(value)
    if object == nil then return "<nil>" end
    local valid_ok, valid = pcall(function() return object:IsValid() end)
    if not valid_ok or not valid then return "<invalid>" end
    local name_ok, name = pcall(function() return object:GetFullName() end)
    if name_ok and name ~= nil then return tostring(name) end
    return "<unnamed>"
end

local function register_native_hook(path, pre_callback, post_callback)
    local ok, pre_id, post_id = pcall(function()
        if post_callback ~= nil then
            return RegisterHook(path, pre_callback, post_callback)
        end
        return RegisterHook(path, pre_callback)
    end)
    if not ok then
        log_event("HOOK_FAILED", string.format("path=%s error=%s", path, tostring(pre_id)))
        return false
    end
    hook_ids[path] = { pre_id, post_id }
    log_event("HOOK_REGISTERED", string.format("path=%s pre=%s post=%s",
        path, tostring(pre_id), tostring(post_id)))
    return true
end

local function trace_parameterless(name)
    return function(context)
        log_event(name .. "_PRE", "context=" .. safe_name(context))
        return nil
    end
end

local function trace_bool_result(name)
    return function(context, return_value)
        log_event(name .. "_POST", string.format("context=%s native_return=%s",
            safe_name(context), safe_bool(return_value)))
        return nil
    end
end

local function init_trace()
    register_native_hook(
        MANAGER_PATH .. "IsPlayable",
        trace_parameterless("IS_PLAYABLE"),
        trace_bool_result("IS_PLAYABLE")
    )
    register_native_hook(
        MANAGER_PATH .. "IsModeStart",
        trace_parameterless("IS_MODE_START"),
        trace_bool_result("IS_MODE_START")
    )
    register_native_hook(MANAGER_PATH .. "OnListUp", trace_parameterless("LIST_UP"))
    register_native_hook(MANAGER_PATH .. "OnListDown", trace_parameterless("LIST_DOWN"))
    register_native_hook(MANAGER_PATH .. "SetButtonFocus", trace_parameterless("SET_BUTTON_FOCUS"))
    register_native_hook(MENU_PATH .. "DecideButton", trace_parameterless("DECIDE_BUTTON"))
    register_native_hook(
        MENU_PATH .. "NewDecideButton",
        function(context, menu_button)
            log_event("NEW_DECIDE_BUTTON_PRE", string.format("context=%s button=%s",
                safe_name(context), safe_name(menu_button)))
            return nil
        end
    )
    log_event("TRACE_READY", "mode=read-only")
end

log_event("INITIALIZING", "version=gate-1")
ExecuteInGameThread(init_trace)
