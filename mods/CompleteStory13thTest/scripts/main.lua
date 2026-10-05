-- Complete Story: 13th Episode Battle registry prototype v0.4
--
-- This prototype never edits cooked files or saves. F8 is read-only. F9 performs
-- one guarded, in-memory alias insertion. F10 removes that insertion in the same
-- session. Closing the game also discards the in-memory change.

local PREFIX = "[CompleteStory v0.4] "
local REGISTRY_PATH = "/Game/SS/Blueprints/DragonAdventureIFData.DragonAdventureIFData"
local REGISTRY_CLASS = "SSDragonAdventureIFDataAsset"
local GOKU_ASSET_TOKEN = "/Game/SS/MasterDataAsset/DragonAdventureIF/0000_00/DAIF_CharaData_0000_00"

-- Existing, valid Korat character-data key. FNAME_Find is used below so this
-- prototype cannot create an invented key. The stock Episode registry is
-- expected not to contain this key; F8 verifies that before F9 can arm.
local TEST_KEY_NAME = "0000_00"
local STOCK_ENTRY_COUNT = 12

local armed = false
local inserted_by_this_session = false

local function log(message)
    print(PREFIX .. tostring(message) .. "\n")
end

local function try(label, callback)
    local ok, value = pcall(callback)
    if not ok then
        log("FAIL " .. label .. ": " .. tostring(value))
        return false, nil
    end
    return true, value
end

local function valid_object(object)
    if object == nil then return false end
    local ok, result = pcall(function() return object:IsValid() end)
    return ok and result == true
end

local function object_name(object)
    if not valid_object(object) then return "<invalid UObject>" end
    local ok, result = pcall(function() return object:GetFullName() end)
    if ok then return tostring(result) end
    return tostring(object)
end

local function key_name(key)
    local ok, result = pcall(function() return key.Key:ToString() end)
    if ok then return tostring(result) end
    return "<unreadable key: " .. tostring(result) .. ">"
end

local function find_registry()
    local ok, registry = try("StaticFindObject registry", function()
        return StaticFindObject(REGISTRY_PATH)
    end)
    if ok and valid_object(registry) then return registry end

    log("Registry was not found by path; trying the loaded non-default instance")
    ok, registry = try("FindFirstOf registry class", function()
        return FindFirstOf(REGISTRY_CLASS)
    end)
    if ok and valid_object(registry) then return registry end
    return nil
end

local function existing_test_key()
    local ok, name = try("resolve existing test FName", function()
        return FName(TEST_KEY_NAME, EFindName.FNAME_Find)
    end)
    if not ok or name == nil then return nil end

    local ok_text, text = pcall(function() return name:ToString() end)
    if not ok_text or tostring(text) ~= TEST_KEY_NAME then
        log("BLOCKED: " .. TEST_KEY_NAME .. " is not present in the existing FName pool")
        return nil
    end
    return { Key = name }
end

local function inspect(require_stock_baseline)
    local registry = find_registry()
    if registry == nil then
        log("BLOCKED: DragonAdventureIFData is not loaded")
        return nil
    end

    local ok, map = try("read PtrRecords", function() return registry.PtrRecords end)
    if not ok or map == nil then return nil end

    local count_ok, count = try("count PtrRecords", function() return #map end)
    if not count_ok then return nil end
    log("PtrRecords count = " .. tostring(count))

    local goku_value = nil
    local goku_key_text = nil
    local iteration_ok = select(1, try("enumerate PtrRecords", function()
        map:ForEach(function(key_param, value_param)
            local key = key_param:get()
            local value = value_param:get()
            local key_text = key_name(key)
            local value_text = object_name(value)
            log("entry key=" .. key_text .. " value=" .. value_text)

            if string.find(value_text, GOKU_ASSET_TOKEN, 1, true) then
                if goku_value ~= nil then
                    error("more than one registry entry points to the expected Goku asset")
                end
                goku_value = value
                goku_key_text = key_text
            end
        end)
        return true
    end))
    if not iteration_ok then return nil end

    if not valid_object(goku_value) then
        log("BLOCKED: exact Goku Episode Battle asset was not found in PtrRecords")
        return nil
    end
    log("Goku route discovered at key " .. tostring(goku_key_text))

    local test_key = existing_test_key()
    if test_key == nil then return nil end

    local contains_ok, contains = try("check test key", function()
        return map:Contains(test_key)
    end)
    if not contains_ok then return nil end

    if require_stock_baseline then
        if count ~= STOCK_ENTRY_COUNT then
            log("BLOCKED: expected " .. STOCK_ENTRY_COUNT .. " entries; another mod/build changed the registry")
            return nil
        end
        if contains then
            log("BLOCKED: test key " .. TEST_KEY_NAME .. " is already registered")
            return nil
        end
    end

    return {
        registry = registry,
        map = map,
        count = count,
        goku_value = goku_value,
        goku_key_text = goku_key_text,
        test_key = test_key,
        test_key_present = contains,
    }
end

local function run_f8()
    armed = false
    log("F8 read-only diagnostic started")
    local state = inspect(true)
    if state == nil then
        log("F8 did not arm F9; no changes made")
        return
    end
    armed = true
    log("F8 PASSED; F9 is armed for one in-memory insertion")
    log("No changes were made")
end

local function run_f9()
    if inserted_by_this_session then
        log("F9 BLOCKED: this session already inserted the test entry")
        return
    end
    if not armed then
        log("F9 BLOCKED: run F8 and obtain a PASS first")
        return
    end

    -- Re-check immediately before mutation; do not trust stale state from F8.
    local state = inspect(true)
    if state == nil then
        armed = false
        log("F9 BLOCKED: pre-insertion checks changed or failed")
        return
    end

    local add_ok = select(1, try("add temporary registry alias", function()
        state.map:Add(state.test_key, state.goku_value)
        return true
    end))
    if not add_ok then
        armed = false
        return
    end

    local count_ok, new_count = try("verify new count", function() return #state.map end)
    local contains_ok, contains = try("verify inserted key", function()
        return state.map:Contains(state.test_key)
    end)

    if not count_ok or new_count ~= STOCK_ENTRY_COUNT + 1 or not contains_ok or not contains then
        log("POST-CHECK FAILED; attempting immediate rollback")
        pcall(function() state.map:Remove(state.test_key) end)
        armed = false
        return
    end

    inserted_by_this_session = true
    armed = false
    log("INSERTED: PtrRecords now has " .. tostring(new_count) .. " entries")
    log("This is a Goku-data alias, not yet a separately named Complete Story asset")
    log("Open Episode Battle once and inspect the selector; press Ctrl+Alt+F10 to roll back")
end

local function run_f10()
    if not inserted_by_this_session then
        log("F10: nothing owned by this mod session to remove")
        return
    end

    local registry = find_registry()
    if registry == nil then
        log("F10 FAILED: registry unavailable; closing the game will discard the transient entry")
        return
    end
    local ok, map = try("read PtrRecords for rollback", function() return registry.PtrRecords end)
    if not ok or map == nil then return end
    local test_key = existing_test_key()
    if test_key == nil then return end

    local remove_ok = select(1, try("remove temporary registry alias", function()
        map:Remove(test_key)
        return true
    end))
    if not remove_ok then return end

    local count_ok, count = try("verify rollback count", function() return #map end)
    if count_ok and count == STOCK_ENTRY_COUNT then
        inserted_by_this_session = false
        log("ROLLBACK PASSED: PtrRecords restored to " .. tostring(count) .. " entries")
    else
        log("ROLLBACK UNVERIFIED: do not continue; close the game to clear transient state")
    end
end

RegisterKeyBind(Key.F8, { ModifierKey.CONTROL, ModifierKey.ALT }, function()
    ExecuteInGameThread(function()
        local ok, err = pcall(run_f8)
        if not ok then log("UNEXPECTED F8 ERROR: " .. tostring(err)) end
    end)
end)

RegisterKeyBind(Key.F9, { ModifierKey.CONTROL, ModifierKey.ALT }, function()
    ExecuteInGameThread(function()
        local ok, err = pcall(run_f9)
        if not ok then log("UNEXPECTED F9 ERROR: " .. tostring(err)) end
    end)
end)

RegisterKeyBind(Key.F10, { ModifierKey.CONTROL, ModifierKey.ALT }, function()
    ExecuteInGameThread(function()
        local ok, err = pcall(run_f10)
        if not ok then log("UNEXPECTED F10 ERROR: " .. tostring(err)) end
    end)
end)

log("Loaded. Ctrl+Alt+F8=inspect; F9=guarded add; F10=rollback")
