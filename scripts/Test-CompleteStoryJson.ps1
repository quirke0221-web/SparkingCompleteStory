param([Parameter(Mandatory)][string]$JsonDirectory)

$ErrorActionPreference = 'Stop'
function Read-Json([string]$name) { Get-Content -LiteralPath (Join-Path $JsonDirectory $name) -Raw | ConvertFrom-Json -Depth 100 }
function Property($asset, [string]$name) { $asset.Exports[0].Data | Where-Object Name -eq $name }

$registry = Read-Json 'DragonAdventureIFData.modified.json'
$chart = Read-Json 'DragonAdventureIFChartData.modified.json'
$character = Read-Json 'DAIF_CharaData_CompleteStory.json'

$records = Property $registry 'PtrRecords'
$chartRecords = Property $chart 'PtrRecords'
$keys = @($records.Value | ForEach-Object { $_[0].Value[0].Value })
$chartKeys = @($chartRecords.Value | ForEach-Object { $_[0].Value[0].Value })
$default = Property $registry 'DefaultOpenCharacter'

if ($records.Value.Count -ne 13 -or ($keys | Select-Object -Unique).Count -ne 13) { throw 'Character registry is not exactly 13 unique records.' }
if ($chartRecords.Value.Count -ne 13 -or ($chartKeys | Select-Object -Unique).Count -ne 13) { throw 'Chart registry is not exactly 13 unique records.' }
if ($keys[-1] -ne '0000_00' -or $chartKeys[-1] -ne '0000_00') { throw 'Complete Story key is missing from one or both registries.' }
if ($default.Value[0].Value -ne '0000_40') { throw 'DefaultOpenCharacter must remain 0000_40.' }
if ((Property $character 'StartEvent').Value -ne 'Event_00_0_00_00') { throw 'Unexpected start event.' }
if ((Property $character 'StartEventBlock').Value -ne 'EventBlock_0000_00') { throw 'Unexpected start event block.' }
if ((Property $character 'IgnoreOpen')) { throw 'IgnoreOpen must not be present in the clean v0.3 baseline.' }

[pscustomobject]@{
    CharacterRecords = $records.Value.Count
    ChartRecords = $chartRecords.Value.Count
    DefaultOpenCharacter = $default.Value[0].Value
    StartEvent = (Property $character 'StartEvent').Value
    StartEventBlock = (Property $character 'StartEventBlock').Value
    Result = 'PASS (structural only; runtime playability is not proven)'
}
