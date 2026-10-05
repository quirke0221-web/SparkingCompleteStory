param(
    [Parameter(Mandatory)][string]$OutputDirectory,
    [string]$ConfigPath = (Join-Path $PSScriptRoot '..\config\project.local.psd1'),
    [string[]]$Filters = @(
        'SparkingZERO/Content/SS/Blueprints/DragonAdventureIFData',
        'SparkingZERO/Content/SS/Blueprints/DragonAdventureIFChartData',
        'SparkingZERO/Content/SS/MasterDataAsset/DragonAdventureIF/0000_00/DAIF_CharaData_0000_00'
    )
)

. (Join-Path $PSScriptRoot 'Common.ps1')
$config = Get-ProjectConfiguration $ConfigPath
Assert-File $config.RetocPath 'retoc'
$paks = Join-Path $config.GameRoot 'SparkingZERO\Content\Paks'
if (-not (Test-Path -LiteralPath $paks -PathType Container)) { throw "Game Paks directory not found: $paks" }
$aes = [Environment]::GetEnvironmentVariable($config.AesKeyEnvironmentVariable)
if ([string]::IsNullOrWhiteSpace($aes)) { throw "Set environment variable $($config.AesKeyEnvironmentVariable) for this shell. It is never saved by this script." }

New-Item -ItemType Directory -Force -Path $OutputDirectory | Out-Null
foreach ($filter in $Filters) {
    # retoc's filter is applied during conversion, avoiding a whole-game extraction.
    Invoke-Checked $config.RetocPath @('--aes-key', $aes, 'to-legacy', '--version', 'UE5_1', '--filter', $filter, $paks, $OutputDirectory)
}

Write-Output 'Targeted extraction complete. Inspect the output and copy only required packages into a clean staging directory.'
