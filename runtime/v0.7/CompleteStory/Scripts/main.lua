local TAG = "[CompleteStory v0.7-dev]"
local TARGET_INDEX = 12
local HOOK_DELAY_MS = 60000
local last_signature = ""
local entered_logged = false
local hook_installed = false

local function log(message)
    print(string.format("%s %s\n", TAG, message))
end

local function get_context(context)
    if context == nil then
        return nil
    end

    local ok, value = pcall(function()
        return context:get()
    end)
    if ok and value ~= nil then
        return value
    end

    return context
end

local function read_panel(manager, slot)
    local property_name = string.format("WBP_OBJ_AI_CharacterPanel_%d", slot)
    local ok, panel = pcall(function()
        return manager[property_name]
    end)
    if not ok or panel == nil then
        return "-"
    end

    local index_ok, index = pcall(function()
        return panel.Index
    end)
    local show_ok, show_num = pcall(function()
        return panel.ShowNum
    end)

    if not index_ok then
        return "?"
    end

    return string.format("%s/%s", tostring(index), show_ok and tostring(show_num) or "?")
end

local function selection_signature(manager)
    local values = {}
    for slot = 0, 5 do
        values[#values + 1] = read_panel(manager, slot)
    end
    return table.concat(values, ",")
end

local function current_primary_index(manager)
    local ok, panel = pcall(function()
        return manager.WBP_OBJ_AI_CharacterPanel_0
    end)
    if not ok or panel == nil then
        return nil
    end

    local index_ok, index = pcall(function()
        return panel.Index
    end)
    if not index_ok then
        return nil
    end

    return tonumber(index)
end

local function is_playable_post(context)
    local manager = get_context(context)
    if manager == nil then
        return nil
    end

    if not entered_logged then
        entered_logged = true
        log("IsPlayable callback entered")
    end

    local signature = selection_signature(manager)
    if signature ~= last_signature then
        last_signature = signature
        log("panel Index/ShowNum slots 0..5: " .. signature)
    end

    local primary_index = current_primary_index(manager)
    if TARGET_INDEX ~= nil and primary_index == TARGET_INDEX then
        log(string.format("playability override applied for verified Complete Story index %d", TARGET_INDEX))
        return true
    end

    return nil
end

local function install_hook()
    if hook_installed then
        return
    end

    local ok, pre_id, post_id = pcall(function()
        return RegisterHook(
            "/Script/SS.SSDragonAdventureIFCSManager:IsPlayable",
            function(context) return nil end,
            is_playable_post
        )
    end)

    if ok then
        hook_installed = true
        log(string.format("IsPlayable hook installed on game thread (ids=%s,%s)", tostring(pre_id), tostring(post_id)))
    else
        log("FATAL: delayed IsPlayable hook installation failed: " .. tostring(pre_id))
    end
end

log(string.format("mod initialized; delayed safe hook scheduled for %d ms", HOOK_DELAY_MS))
ExecuteWithDelay(HOOK_DELAY_MS, function()
    ExecuteInGameThread(install_hook)
end)
