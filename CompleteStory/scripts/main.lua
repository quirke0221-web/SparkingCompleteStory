-- Dragon Ball: Sparking! ZERO - Complete Story
-- Zero Silent Failures Instrumentation & In-Memory Unlock Probe
-- Target: RE-UE4SS v3.0.1 Beta

local MOD_TAG = "[CompleteStory-Audit]"
local sequence = 0

local function log_event(status, details)
    sequence = sequence + 1
    print(string.format("%s TRACE %04d [%s] %s\n", MOD_TAG, sequence, status, details or ""))
end

local function resolve_uobject(param)
    if param == nil then return nil end
    local ok, inner = pcall(function() return param:get() end)
    local obj = (ok and inner) or param
    if obj and type(obj) == "userdata" then
        local valid_ok, is_valid = pcall(function() return obj:IsValid() end)
        if valid_ok and is_valid then
            return obj
        end
    end
    return nil
end

local function safe_obj_name(param)
    local obj = resolve_uobject(param)
    if not obj then return "<nil/invalid>" end
    local ok, name = pcall(function() return obj:GetFullName() end)
    if ok and name ~= nil then
        return tostring(name)
    end
    return "<unnamed>"
end

-- Re-entrancy guard to prevent recursive invocation loops
local in_hook = false

-- Hook SSDragonAdventureIFCSManager:IsPlayable (Pre & Post passive telemetry)
local ok_hook, hook_err = pcall(function()
    return RegisterHook(
        "/Script/SS.SSDragonAdventureIFCSManager:IsPlayable",
        -- Pre-callback: Safe passive observation
        function(context)
            if in_hook then return end
            in_hook = true

            local hook_ok, hook_err_msg = pcall(function()
                log_event("HOOK_INVOKED", "IsPlayable PRE on " .. safe_obj_name(context))
            end)

            if not hook_ok then
                log_event("HOOK_RUNTIME_ERROR", "Exception in hook pre-body: " .. tostring(hook_err_msg))
            end

            in_hook = false
        end,
        -- Post-callback: Observe native engine return value safely without recursion
        function(context, return_value)
            local ret_str = "<nil>"
            if return_value ~= nil then
                local ok, val = pcall(function() return return_value:get() end)
                ret_str = (ok and tostring(val)) or tostring(return_value)
            end
            log_event("NATIVE_RETURN", "IsPlayable POST returned: " .. ret_str)
        end
    )
end)

if not ok_hook then
    log_event("FATAL_HOOK_FAIL", "Failed to register IsPlayable hook: " .. tostring(hook_err))
else
    log_event("HOOK_READY", "Zero-silent-failure probe attached with re-entrancy protection")
end
