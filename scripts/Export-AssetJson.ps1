param(
    [Parameter(Mandatory)][string]$InputDirectory,
    [Parameter(Mandatory)][string]$OutputDirectory,
    [string]$ConfigPath = (Join-Path $PSScriptRoot '..\config\project.local.psd1')
)

. (Join-Path $PSScriptRoot 'Common.ps1')
$config = Get-ProjectConfiguration $ConfigPath
Assert-File $config.UAssetGUIPath 'UAssetGUI'
Assert-File $config.MappingPath 'Sparking ZERO mapping'

$assets = @(
    'SparkingZERO\Content\SS\Blueprints\DragonAdventureIFData.uasset',
    'SparkingZERO\Content\SS\Blueprints\DragonAdventureIFChartData.uasset',
    'SparkingZERO\Content\SS\MasterDataAsset\DragonAdventureIF\0000_00\DAIF_CharaData_0000_00.uasset'
)

New-Item -ItemType Directory -Force -Path $OutputDirectory | Out-Null
foreach ($relativePath in $assets) {
    $source = Join-Path $InputDirectory $relativePath
    Assert-File $source "Required legacy asset '$relativePath'"
    $destination = Join-Path $OutputDirectory (([IO.Path]::GetFileNameWithoutExtension($source)) + '.json')
    Invoke-Checked $config.UAssetGUIPath @('tojson', $source, $destination, 'VER_UE5_1', $config.MappingName)
}
