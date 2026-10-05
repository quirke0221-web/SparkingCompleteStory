<#
.SYNOPSIS
    Transforms Dragon Ball Sparking! ZERO serialized asset JSON to register Complete Story.
.DESCRIPTION
    Pure domain logic for modifying serialized JSON assets:
    - Slices route key 0000_00 into DragonAdventureIFData registry
    - Clones Goku DAIF_CharaData_0000_00 into DAIF_CharaData_CompleteStory
    - Links 0000_00 to ChartData0000_00 in DragonAdventureIFChartData
    Zero external tool subprocess calls.
#>
param(
    [Parameter(Mandatory = $true)]
    [string]$RegistryJson,

    [Parameter(Mandatory = $true)]
    [string]$GokuJson,

    [Parameter(Mandatory = $true)]
    [string]$ChartRegistryJson,

    [Parameter(Mandatory = $true)]
    [string]$OutputDirectory
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$routeKey       = '0000_00'
$newObjectName  = 'DAIF_CharaData_CompleteStory'
$oldObjectName  = 'DAIF_CharaData_0000_00'
$oldPackageName = '/Game/SS/MasterDataAsset/DragonAdventureIF/0000_00/DAIF_CharaData_0000_00'
$newPackageName = '/Game/SS/MasterDataAsset/DragonAdventureIF/CompleteStory/DAIF_CharaData_CompleteStory'

function Read-UAssetJson([string]$Path) {
    return Get-Content -LiteralPath $Path -Raw | ConvertFrom-Json -Depth 100
}

function Clone-JsonObject($Value) {
    return $Value | ConvertTo-Json -Depth 100 -Compress | ConvertFrom-Json -Depth 100
}

function Write-UAssetJson([string]$Path, $Value) {
    $parent = Split-Path -Parent $Path
    if (-not (Test-Path -LiteralPath $parent)) {
        $null = New-Item -ItemType Directory -Force -Path $parent
    }
    $json = $Value | ConvertTo-Json -Depth 100
    [System.IO.File]::WriteAllText($Path, $json, [System.Text.UTF8Encoding]::new($false))
}

$registry      = Read-UAssetJson $RegistryJson
$character     = Read-UAssetJson $GokuJson
$chartRegistry = Read-UAssetJson $ChartRegistryJson

# --- 1. Validate & Transform Character Data Asset ---
if ($character.Exports.Count -ne 1 -or $character.Exports[0].ObjectName -ne $oldObjectName) {
    throw 'Unexpected Goku character-data export layout'
}

$characterProps = $character.Exports[0].Data
$startEvent = $characterProps | Where-Object Name -eq 'StartEvent'
$startBlock = $characterProps | Where-Object Name -eq 'StartEventBlock'
$eventData  = $characterProps | Where-Object Name -eq 'EventData'
if ($startEvent.Value -ne 'Event_00_0_00_00' -or $startBlock.Value -ne 'EventBlock_0000_00' -or [int]$eventData.Value -ne -7) {
    throw 'Goku Raditz start pointers do not match the verified baseline'
}

for ($i = 0; $i -lt $character.NameMap.Count; $i++) {
    if ($character.NameMap[$i] -eq $oldObjectName) { $character.NameMap[$i] = $newObjectName }
    elseif ($character.NameMap[$i] -eq $oldPackageName) { $character.NameMap[$i] = $newPackageName }
}
$character.Exports[0].ObjectName = $newObjectName
$character.FolderName = $newPackageName
$character.Generations[0].NameCount = $character.NameMap.Count

$characterName = $characterProps | Where-Object Name -eq 'CharacterName'
$characterName.Flags = 2 # ETextFlag.CultureInvariant
$characterName.HistoryType = 'None'
$characterName.TableId = $null
$characterName.Namespace = $null
$characterName.CultureInvariantString = 'Complete Story'
$characterName.Value = $null

# --- 2. Validate & Transform Master Character Registry ---
if ($registry.Exports.Count -ne 1 -or $registry.Imports.Count -ne 27) {
    throw "Unexpected registry layout: exports=$($registry.Exports.Count), imports=$($registry.Imports.Count)"
}
$records = $registry.Exports[0].Data | Where-Object Name -eq 'PtrRecords'
if ($null -eq $records -or $records.Value.Count -ne 12) {
    throw "Expected exactly 12 PtrRecords entries; observed $($records.Value.Count)"
}
$defaultOpen = $registry.Exports[0].Data | Where-Object Name -eq 'DefaultOpenCharacter'
if ($null -eq $defaultOpen -or $defaultOpen.Value[0].Value -ne '0000_40') {
    throw 'Expected stock default-open campaign to be 0000_40'
}
if ($routeKey -in @($records.Value | ForEach-Object { $_[0].Value[0].Value })) {
    throw "Route key $routeKey already exists in registry"
}

$exportNameCount = [int]$registry.NamesReferencedFromExportDataCount
$head = @($registry.NameMap[0..($exportNameCount - 1)])
$tail = @($registry.NameMap[$exportNameCount..($registry.NameMap.Count - 1)])
$registry.NameMap = @($head + $routeKey + $tail + $newPackageName + $newObjectName)
$registry.NamesReferencedFromExportDataCount = $exportNameCount + 1
$registry.Generations[0].NameCount = $registry.NameMap.Count

$pkgImportIdx = $registry.Imports.Count
$pkgImport = Clone-JsonObject $registry.Imports[1]
$pkgImport.ObjectName = $newPackageName
$pkgImport.OuterIndex = 0
$pkgImport.ClassPackage = '/Script/CoreUObject'
$pkgImport.ClassName = 'Package'
$pkgImport.PackageName = $null
$pkgImport.bImportOptional = $false
$registry.Imports = @($registry.Imports + $pkgImport)

$objImportIdx = $registry.Imports.Count
$objImport = Clone-JsonObject $registry.Imports[14]
$objImport.ObjectName = $newObjectName
$objImport.OuterIndex = -($pkgImportIdx + 1)
$objImport.ClassPackage = '/Script/SS'
$objImport.ClassName = 'SSDragonAdventureIFCharacterDataAsset'
$objImport.PackageName = $null
$objImport.bImportOptional = $false
$registry.Imports = @($registry.Imports + $objImport)

$newRecord = Clone-JsonObject $records.Value[0]
$newRecord[0].Value[0].Value = $routeKey
$newRecord[1].Value = -($objImportIdx + 1)
$records.Value = @($records.Value + (, $newRecord))

$deps = @($registry.Exports[0].CreateBeforeCreateDependencies)
$registry.Exports[0].CreateBeforeCreateDependencies = @($deps + (-($objImportIdx + 1)))

if ($records.Value.Count -ne 13 -or $registry.Imports.Count -ne 29) {
    throw "Registry postcondition failed: records=$($records.Value.Count), imports=$($registry.Imports.Count)"
}

# --- 3. Validate & Transform Chart Registry ---
if ($chartRegistry.Exports.Count -ne 1) {
    throw "Unexpected chart registry layout: exports=$($chartRegistry.Exports.Count)"
}
$chartRecords = $chartRegistry.Exports[0].Data | Where-Object Name -eq 'PtrRecords'
if ($null -eq $chartRecords -or $chartRecords.Value.Count -ne 12) {
    throw "Expected exactly 12 chart PtrRecords entries; observed $($chartRecords.Value.Count)"
}
if ($routeKey -in @($chartRecords.Value | ForEach-Object { $_[0].Value[0].Value })) {
    throw "Chart route key $routeKey already exists"
}
$gokuChartRecord = $chartRecords.Value | Where-Object { $_[0].Value[0].Value -eq '0000_40' }
if ($null -eq $gokuChartRecord -or $gokuChartRecord.Count -ne 2) {
    throw 'Goku chart record was not found'
}
$chartExportNameCount = [int]$chartRegistry.NamesReferencedFromExportDataCount
$chartHead = @($chartRegistry.NameMap[0..($chartExportNameCount - 1)])
$chartTail = @($chartRegistry.NameMap[$chartExportNameCount..($chartRegistry.NameMap.Count - 1)])
$chartRegistry.NameMap = @($chartHead + $routeKey + $chartTail)
$chartRegistry.NamesReferencedFromExportDataCount = $chartExportNameCount + 1
$chartRegistry.Generations[0].NameCount = $chartRegistry.NameMap.Count

$newChartRecord = Clone-JsonObject $gokuChartRecord
$newChartRecord[0].Value[0].Value = $routeKey
$chartRecords.Value = @($chartRecords.Value + (, $newChartRecord))

if ($chartRecords.Value.Count -ne 13) {
    throw 'Chart registry postcondition failed'
}

# --- 4. Write Output JSONs ---
$regOut   = Join-Path $OutputDirectory 'DragonAdventureIFData.modified.json'
$charOut  = Join-Path $OutputDirectory 'DAIF_CharaData_CompleteStory.json'
$chartOut = Join-Path $OutputDirectory 'DragonAdventureIFChartData.modified.json'

Write-UAssetJson $regOut $registry
Write-UAssetJson $charOut $character
Write-UAssetJson $chartOut $chartRegistry

[PSCustomObject]@{
    RegistryJson         = $regOut
    CharacterJson        = $charOut
    ChartRegistryJson    = $chartOut
    RouteKey             = $routeKey
    RecordCount          = $records.Value.Count
    ImportCount          = $registry.Imports.Count
    DefaultOpenCharacter = $defaultOpen.Value[0].Value
    ChartRecordCount     = $chartRecords.Value.Count
}
