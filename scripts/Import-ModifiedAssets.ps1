param(
    [Parameter(Mandatory)][string]$JsonDirectory,
    [Parameter(Mandatory)][string]$StageDirectory,
    [string]$ConfigPath = (Join-Path $PSScriptRoot '..\config\project.local.psd1')
)

. (Join-Path $PSScriptRoot 'Common.ps1')
$config = Get-ProjectConfiguration $ConfigPath
Assert-File $config.UAssetGUIPath 'UAssetGUI'

$outputs = @(
    @{ Json='DragonAdventureIFData.modified.json'; Relative='SparkingZERO\Content\SS\Blueprints\DragonAdventureIFData.uasset' },
    @{ Json='DragonAdventureIFChartData.modified.json'; Relative='SparkingZERO\Content\SS\Blueprints\DragonAdventureIFChartData.uasset' },
    @{ Json='DAIF_CharaData_CompleteStory.json'; Relative='SparkingZERO\Content\SS\MasterDataAsset\DragonAdventureIF\CompleteStory\DAIF_CharaData_CompleteStory.uasset' }
)

foreach ($item in $outputs) {
    $source = Join-Path $JsonDirectory $item.Json
    $destination = Join-Path $StageDirectory $item.Relative
    Assert-File $source "Modified JSON '$($item.Json)'"
    New-Item -ItemType Directory -Force -Path (Split-Path -Parent $destination) | Out-Null
    Invoke-Checked $config.UAssetGUIPath @('fromjson', $source, $destination, $config.MappingName)
}
