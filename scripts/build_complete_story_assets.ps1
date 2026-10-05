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

$ErrorActionPreference = 'Stop'

$routeKey = '0000_00'
$newObjectName = 'DAIF_CharaData_CompleteStory'
$oldObjectName = 'DAIF_CharaData_0000_00'
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
    New-Item -ItemType Directory -Force -Path $parent | Out-Null
    $json = $Value | ConvertTo-Json -Depth 100
    [System.IO.File]::WriteAllText($Path, $json, [System.Text.UTF8Encoding]::new($false))
}

$registry = Read-UAssetJson $RegistryJson
$character = Read-UAssetJson $GokuJson
$chartRegistry = Read-UAssetJson $ChartRegistryJson

if ($registry.Exports.Count -ne 1 -or $registry.Imports.Count -ne 27) {
    throw "Unexpected registry layout: exports=$($registry.Exports.Count), imports=$($registry.Imports.Count)"
}

$records = $registry.Exports[0].Data | Where-Object Name -eq 'PtrRecords'
if ($null -eq $records -or $records.Value.Count -ne 12) {
    throw "Expected exactly 12 PtrRecords entries; observed $($records.Value.Count)"
}

$defaultOpenCharacter = $registry.Exports[0].Data | Where-Object Name -eq 'DefaultOpenCharacter'
if ($null -eq $defaultOpenCharacter -or $defaultOpenCharacter.Value[0].Value -ne '0000_40') {
    throw 'Expected the stock default-open campaign to be 0000_40'
}

$existingKeys = @($records.Value | ForEach-Object { $_[0].Value[0].Value })
if ($routeKey -in $existingKeys) {
    throw "Route key $routeKey already exists"
}

if ($character.Exports.Count -ne 1 -or $character.Exports[0].ObjectName -ne $oldObjectName) {
    throw 'Unexpected Goku character-data export layout'
}

$characterProperties = $character.Exports[0].Data
$startEvent = $characterProperties | Where-Object Name -eq 'StartEvent'
$startBlock = $characterProperties | Where-Object Name -eq 'StartEventBlock'
$eventData = $characterProperties | Where-Object Name -eq 'EventData'
if ($startEvent.Value -ne 'Event_00_0_00_00' -or
    $startBlock.Value -ne 'EventBlock_0000_00' -or
    [int]$eventData.Value -ne -7) {
    throw 'Goku Raditz start pointers do not match the verified baseline'
}

# Clone Goku as a separately named package. All story/event references remain
# unchanged so the prototype starts at the original canonical Raditz opening.
for ($i = 0; $i -lt $character.NameMap.Count; $i++) {
    if ($character.NameMap[$i] -eq $oldObjectName) {
        $character.NameMap[$i] = $newObjectName
    }
    elseif ($character.NameMap[$i] -eq $oldPackageName) {
        $character.NameMap[$i] = $newPackageName
    }
}

$character.Exports[0].ObjectName = $newObjectName
$character.FolderName = $newPackageName
$character.Generations[0].NameCount = $character.NameMap.Count

$characterName = $characterProperties | Where-Object Name -eq 'CharacterName'
$characterName.Flags = 2 # ETextFlag.CultureInvariant
$characterName.HistoryType = 'None'
$characterName.TableId = $null
$characterName.Namespace = $null
$characterName.CultureInvariantString = 'Complete Story'
$characterName.Value = $null

# Insert the route key among names referenced by export data. Package/object
# names are import-only and are appended after the original name map.
$exportNameCount = [int]$registry.NamesReferencedFromExportDataCount
$head = @($registry.NameMap[0..($exportNameCount - 1)])
$tail = @($registry.NameMap[$exportNameCount..($registry.NameMap.Count - 1)])
$registry.NameMap = @($head + $routeKey + $tail + $newPackageName + $newObjectName)
$registry.NamesReferencedFromExportDataCount = $exportNameCount + 1
$registry.Generations[0].NameCount = $registry.NameMap.Count

# Add a package import and then its public data-asset object import. Import
# indices are zero-based in JSON and encoded as -(index + 1) in FPackageIndex.
$packageImportIndex = $registry.Imports.Count
$packageImport = Clone-JsonObject $registry.Imports[1]
$packageImport.ObjectName = $newPackageName
$packageImport.OuterIndex = 0
$packageImport.ClassPackage = '/Script/CoreUObject'
$packageImport.ClassName = 'Package'
$packageImport.PackageName = $null
$packageImport.bImportOptional = $false
$registry.Imports = @($registry.Imports + $packageImport)

$objectImportIndex = $registry.Imports.Count
$objectImport = Clone-JsonObject $registry.Imports[14]
$objectImport.ObjectName = $newObjectName
$objectImport.OuterIndex = -($packageImportIndex + 1)
$objectImport.ClassPackage = '/Script/SS'
$objectImport.ClassName = 'SSDragonAdventureIFCharacterDataAsset'
$objectImport.PackageName = $null
$objectImport.bImportOptional = $false
$registry.Imports = @($registry.Imports + $objectImport)

$newRecord = Clone-JsonObject $records.Value[0]
$newRecord[0].Value[0].Value = $routeKey
$newRecord[1].Value = -($objectImportIndex + 1)
$records.Value = @($records.Value + (, $newRecord))

# DefaultOpenCharacter is a menu-initialization identity, not a general unlock
# bypass. Keep the stock Goku value; changing it regresses the entire selector.

$dependencies = @($registry.Exports[0].CreateBeforeCreateDependencies)
$registry.Exports[0].CreateBeforeCreateDependencies = @($dependencies + (-($objectImportIndex + 1)))

if ($records.Value.Count -ne 13 -or $registry.Imports.Count -ne 29) {
    throw "Postcondition failed: records=$($records.Value.Count), imports=$($registry.Imports.Count)"
}

# Register the same independent key in the chart selector. Reuse Goku's
# existing ChartData0000_00 object import so its complete chart, event blocks,
# progression data, and Raditz opening remain stock assets.
if ($chartRegistry.Exports.Count -ne 1) {
    throw "Unexpected chart registry layout: exports=$($chartRegistry.Exports.Count)"
}
$chartRecords = $chartRegistry.Exports[0].Data | Where-Object Name -eq 'PtrRecords'
if ($null -eq $chartRecords -or $chartRecords.Value.Count -ne 12) {
    throw "Expected exactly 12 chart PtrRecords entries; observed $($chartRecords.Value.Count)"
}
$chartKeys = @($chartRecords.Value | ForEach-Object { $_[0].Value[0].Value })
if ($routeKey -in $chartKeys) {
    throw "Chart route key $routeKey already exists"
}
$gokuChartRecord = $chartRecords.Value | Where-Object { $_[0].Value[0].Value -eq '0000_40' }
if ($null -eq $gokuChartRecord -or $gokuChartRecord.Count -ne 2) {
    throw 'Goku chart record was not found'
}
$gokuChartImportValue = [int]$gokuChartRecord[1].Value
if ($gokuChartImportValue -ge 0 -or
    $chartRegistry.Imports[-$gokuChartImportValue - 1].ObjectName -ne 'ChartData0000_00') {
    throw 'Goku chart record does not resolve to ChartData0000_00'
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

$registryOutput = Join-Path $OutputDirectory 'DragonAdventureIFData.modified.json'
$characterOutput = Join-Path $OutputDirectory 'DAIF_CharaData_CompleteStory.json'
$chartRegistryOutput = Join-Path $OutputDirectory 'DragonAdventureIFChartData.modified.json'
Write-UAssetJson $registryOutput $registry
Write-UAssetJson $characterOutput $character
Write-UAssetJson $chartRegistryOutput $chartRegistry

[pscustomobject]@{
    RegistryJson = $registryOutput
    CharacterJson = $characterOutput
    ChartRegistryJson = $chartRegistryOutput
    RouteKey = $routeKey
    NewPackage = $newPackageName
    RecordCount = $records.Value.Count
    ImportCount = $registry.Imports.Count
    DefaultOpenCharacter = $defaultOpenCharacter.Value[0].Value
    StartEvent = $startEvent.Value
    StartEventBlock = $startBlock.Value
    ChartRecordCount = $chartRecords.Value.Count
    ChartObject = 'ChartData0000_00'
}
