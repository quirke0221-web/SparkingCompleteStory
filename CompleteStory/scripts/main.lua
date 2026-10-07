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

local function audit_and_inject_map(save_obj, label)
    local obj = resolve_uobject(save_obj)
    if not obj then
        log_event("INVALID_OBJECT", label .. " is nil or invalid UObject")
        return
    end

    local ok_cpd, cpd = pcall(function() return obj.CharacterPlayableData end)
    if not ok_cpd then
        log_event("PROPERTY_ERROR", label .. " failed to read CharacterPlayableData: " .. tostring(cpd))
        return
    end
    if cpd == nil then
        log_event("NULL_PROPERTY", label .. " CharacterPlayableData is nil")
        return
    end

    log_event("PROPERTY_FOUND", label .. " CharacterPlayableData type=" .. type(cpd))

    -- 1. Read vanilla Goku entry ("0000_40")
    local ok_goku, val_goku = pcall(function() return cpd["0000_40"] end)
    if not ok_goku then
        log_event("READ_ERROR", label .. " cpd['0000_40'] thrown: " .. tostring(val_goku))
    elseif val_goku == nil then
        log_event("GOKU_NOT_FOUND", label .. " cpd['0000_40'] returned nil")
    else
        log_event("GOKU_READ_OK", label .. " cpd['0000_40'] = " .. tostring(val_goku))
    end

    -- 2. Read custom entry ("0000_00") before write
    local ok_pre, val_pre = pcall(function() return cpd["0000_00"] end)
    if not ok_pre then
        log_event("PRE_READ_ERROR", label .. " cpd['0000_00'] thrown: " .. tostring(val_pre))
    else
        log_event("PRE_READ_OK", label .. " cpd['0000_00'] before write = " .. tostring(val_pre))
    end

    -- 3. Execute Mutation if template exists
    if ok_goku and val_goku ~= nil then
        local ok_write, write_err = pcall(function()
            cpd["0000_00"] = val_goku
        end)

        if not ok_write then
            log_event("WRITE_ERROR", label .. " assignment cpd['0000_00'] failed: " .. tostring(write_err))
        else
            log_event("WRITE_EXECUTED", label .. " assignment cpd['0000_00'] = val_goku executed")
        end

        -- 4. HARD ASSERTION: Immediate Read-Back Verification
        local ok_post, val_post = pcall(function() return cpd["0000_00"] end)
        if not ok_post then
            log_event("READBACK_ERROR", label .. " readback thrown: " .. tostring(val_post))
        elseif val_post == nil then
            log_event("ASSERTION_FAILED", label .. " cpd['0000_00'] returned nil after write. (Map did not persist key)")
        else
            log_event("ASSERTION_PASSED", label .. " cpd['0000_00'] persisted in memory: " .. tostring(val_post))
        end
    end
end

-- Re-entrancy guard to prevent recursive invocation loops
local in_hook = false

-- Hook SSDragonAdventureIFCSManager:IsPlayable (Pre & Post)
local ok_hook, hook_err = pcall(function()
    return RegisterHook(
        "/Script/SS.SSDragonAdventureIFCSManager:IsPlayable",
        -- Pre-callback: Run memory injection before native logic evaluates
        function(context)
            if in_hook then return end
            in_hook = true

            local hook_ok, hook_err_msg = pcall(function()
                log_event("HOOK_INVOKED", "IsPlayable PRE on " .. safe_obj_name(context))

                -- Audit SSMainGameSaveData
                local saves_main = FindAllOf("SSMainGameSaveData")
                if saves_main and #saves_main > 0 then
                    for idx, s in ipairs(saves_main) do
                        audit_and_inject_map(s, string.format("MainGameSaveData_%d", idx))
                    end
                end

                -- Audit SSSystemSaveData
                local saves_sys = FindAllOf("SSSystemSaveData")
                if saves_sys and #saves_sys > 0 then
                    for idx, s in ipairs(saves_sys) do
                        audit_and_inject_map(s, string.format("SystemSaveData_%d", idx))
                    end
                end
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
