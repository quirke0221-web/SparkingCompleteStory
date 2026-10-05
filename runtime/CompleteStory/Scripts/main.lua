local MOD_TAG = "[CompleteStory v0.6]"
local EXPECTED_DISPLAY_NAME = "complete story"
local CUSTOM_CHARACTER_ASSET = "/Game/SS/MasterDataAsset/DragonAdventureIF/CompleteStory/DAIF_CharaData_CompleteStory.DAIF_CharaData_CompleteStory"

local last_selection_log = nil
local override_was_logged = false
local custom_asset_logged = false

local function log_line(message)
    print(string.format("%s %s\n", MOD_TAG, message))
end

local function unwrap(value)
    if value == nil then
        return nil
    end

    local ok, unwrapped = pcall(function()
        return value:get()
    end)

    if ok and unwrapped ~= nil then
        return unwrapped
    end

    return value
end

local function full_name(object)
    object = unwrap(object)
    if object == nil then
        return nil
    end

    local ok, value = pcall(function()
        return object:GetFullName()
    end)

    if ok then
        return value
    end

    return nil
end

local function object_path(object)
    local name = full_name(object)
    if name == nil then
        return nil
    end

    return name:match("^[^ ]+ (.+)$") or name
end

local function text_to_string(value)
    value = unwrap(value)
    if value == nil then
        return nil
    end

    local ok, result = pcall(function()
        return value:ToString()
    end)
    if ok and result ~= nil then
        return tostring(result)
    end

    ok, result = pcall(function()
        return value:GetText():ToString()
    end)
    if ok and result ~= nil then
        return tostring(result)
    end

    return nil
end

local function normalize(value)
    if value == nil then
        return nil
    end

    value = tostring(value):gsub("^%s+", ""):gsub("%s+$", "")
    value = value:gsub("%s+", " "):lower()
    return value
end

local function add_candidate(candidates, source, value)
    local text = text_to_string(value)
    if text ~= nil and text ~= "" then
        table.insert(candidates, { source = source, text = text })
    end
end

local function selected_name_candidates(manager)
    local candidates = {}

    -- Text_Title is a reflected property on both shipped character-select widgets.
    local ok, title = pcall(function()
        return manager.Text_Title
    end)
    if ok then
        add_candidate(candidates, "Text_Title", title)
    end

    -- CaracterText_0/1 are the two rendered copies of the selected character name.
    -- They are not Blueprint variables, so resolve only descendants of this manager.
    local manager_path = object_path(manager)
    if manager_path ~= nil then
        local ok_all, text_blocks = pcall(function()
            return FindAllOf("TextBlock")
        end)

        if ok_all and text_blocks ~= nil then
            for _, widget in ipairs(text_blocks) do
                local widget_name = full_name(widget)
                if widget_name ~= nil
                    and widget_name:find(manager_path, 1, true) ~= nil
                    and (widget_name:match("%.CaracterText_0$") ~= nil
                        or widget_name:match("%.CaracterText_1$") ~= nil) then
                    add_candidate(candidates, widget_name, widget)
                end
            end
        end
    end

    return candidates
end

local function selected_campaign_key(manager)
    manager = unwrap(manager)
    if manager == nil then
        return nil, "manager unavailable"
    end

    local candidates = selected_name_candidates(manager)
    for _, candidate in ipairs(candidates) do
        if normalize(candidate.text) == EXPECTED_DISPLAY_NAME then
            -- v0.3 registers this unique presentation asset under registry key 0000_00.
            return "0000_00", string.format("%s='%s'", candidate.source, candidate.text)
        end
    end

    local seen = {}
    for _, candidate in ipairs(candidates) do
        table.insert(seen, string.format("%s='%s'", candidate.source, candidate.text))
    end

    if #seen == 0 then
        return nil, "no selected-name widget resolved"
    end

    return nil, table.concat(seen, "; ")
end

local function resolve_character_select_manager(context)
    local candidate = unwrap(context)
    local candidate_name = full_name(candidate)
    if candidate_name ~= nil and candidate_name:find("WBP_GRP_AI_CharacterSelect_C", 1, true) ~= nil then
        return candidate
    end

    local ok, managers = pcall(function()
        return FindAllOf("WBP_GRP_AI_CharacterSelect_C")
    end)
    if not ok or managers == nil then
        return nil
    end

    for _, manager in ipairs(managers) do
        local name = full_name(manager)
        if name ~= nil and name:find("/Engine/Transient.", 1, true) ~= nil then
            return manager
        end
    end

    return nil
end

local function log_custom_asset_once()
    if custom_asset_logged then
        return
    end
    custom_asset_logged = true

    local ok, asset = pcall(function()
        return StaticFindObject(CUSTOM_CHARACTER_ASSET)
    end)

    if ok and asset ~= nil then
        local name_ok, character_name = pcall(function()
            return text_to_string(asset.CharacterName)
        end)
        log_line(string.format(
            "custom asset resolved: %s; CharacterName=%s",
            full_name(asset) or CUSTOM_CHARACTER_ASSET,
            name_ok and (character_name or "<unreadable>") or "<unreadable>"
        ))
    else
        log_line("custom asset was not loaded when the first playability check ran")
    end
end

local function is_playable_post(context)
    local manager = unwrap(context)
    log_custom_asset_once()

    local key, evidence = selected_campaign_key(manager)
    local state = string.format("key=%s evidence=%s", key or "<native>", evidence)
    if state ~= last_selection_log then
        last_selection_log = state
        log_line("IsPlayable selection: " .. state)
    end

    if key == "0000_00" then
        if not override_was_logged then
            override_was_logged = true
            log_line("overriding IsPlayable return value to true for Complete Story (0000_00)")
        end
        return true
    end

    override_was_logged = false

    -- A nil return preserves the game's original result for all 12 stock campaigns.
    return nil
end

local function diagnostic_hook(label)
    return function(context, ...)
        local manager = resolve_character_select_manager(context)
        local key, evidence = selected_campaign_key(manager)
        log_line(string.format("%s: key=%s evidence=%s", label, key or "<native>", evidence))
        return nil
    end
end

local function register_optional_hook(path, pre_callback, post_callback)
    local ok, first_id, second_id = pcall(function()
        return RegisterHook(path, pre_callback, post_callback)
    end)

    if ok then
        log_line(string.format("hook installed: %s (ids=%s,%s)", path, tostring(first_id), tostring(second_id)))
    else
        log_line(string.format("hook failed: %s (%s)", path, tostring(first_id)))
    end

    return ok
end

log_line("loading; scoped target is registry key 0000_00 only")

local playable_installed = register_optional_hook(
    "/Script/SS.SSDragonAdventureIFCSManager:IsPlayable",
    function(context) return nil end,
    is_playable_post
)

if not playable_installed then
    log_line("FATAL: IsPlayable hook was not installed; no game behavior will be changed")
end

-- These hooks are diagnostic only. They never alter parameters or return values.
register_optional_hook(
    "/Script/SS.SSBuiltInMenu:DecideButton",
    diagnostic_hook("SSBuiltInMenu.DecideButton"),
    function(context) return nil end
)

register_optional_hook(
    "/Script/SS.SSBuiltInMenu:NewDecideButton",
    diagnostic_hook("SSBuiltInMenu.NewDecideButton"),
    function(context) return nil end
)
