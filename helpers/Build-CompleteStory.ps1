<#
.SYNOPSIS
    Master orchestrator for building and packaging the Complete Story mod.
.DESCRIPTION
    Executes the 6-stage asset build and container packaging pipeline:
    1. Extract required stock assets via retoc to-legacy
    2. Deserialize uassets to JSON via UAssetGUI tojson
    3. Transform JSON assets via Transform-CompleteStoryAssets.ps1
    4. Compile modified JSON back to uassets via UAssetGUI fromjson
    5. Pack into IoStore container (.pak/.utoc/.ucas) via retoc to-zen and verify
    6. Package into an Unverum-ready release zip
#>
param(
    [string]$ConfigPath = (Join-Path $PSScriptRoot '..\config\project.local.psd1'),
    [string]$ContainerName = 'CompleteStory_P',
    [switch]$SkipExtraction,
    [switch]$PackageRelease = $true,
    [switch]$Deploy
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

. (Join-Path $PSScriptRoot 'Common.ps1')
$config = Get-ProjectConfiguration $ConfigPath
$dirs   = Get-PipelineDirectories

Assert-File $config.RetocPath 'retoc executable'
Assert-File $config.UAssetGUIPath 'UAssetGUI executable'
Assert-File $config.MappingPath 'Sparking ZERO usmap file'

$paksDir = Join-Path $config.GameRoot 'SparkingZERO\Content\Paks'
if (-not $SkipExtraction -and -not (Test-Path -LiteralPath $paksDir -PathType Container)) {
    throw "Game Paks directory not found: $paksDir"
}

# --- Stage 1: Targeted Extraction of Stock Assets ---
if (-not $SkipExtraction) {
    $aes = [Environment]::GetEnvironmentVariable($config.AesKeyEnvironmentVariable)
    if ([string]::IsNullOrWhiteSpace($aes)) {
        throw "Environment variable $($config.AesKeyEnvironmentVariable) is not set."
    }
    Ensure-CleanDirectory $dirs.LegacyStaging
    $filters = @(
        'SparkingZERO/Content/SS/Blueprints/DragonAdventureIFData',
        'SparkingZERO/Content/SS/Blueprints/DragonAdventureIFChartData',
        'SparkingZERO/Content/SS/MasterDataAsset/DragonAdventureIF/0000_00/DAIF_CharaData_0000_00'
    )
    foreach ($filter in $filters) {
        Invoke-Checked $config.RetocPath @('--aes-key', $aes, 'to-legacy', '--version', 'UE5_1', '--filter', $filter, $paksDir, $dirs.LegacyStaging)
    }
}

# --- Stage 2: Deserialize Legacy Assets to JSON ---
Ensure-CleanDirectory $dirs.JsonStaging
$stockAssets = @(
    'SparkingZERO\Content\SS\Blueprints\DragonAdventureIFData.uasset',
    'SparkingZERO\Content\SS\Blueprints\DragonAdventureIFChartData.uasset',
    'SparkingZERO\Content\SS\MasterDataAsset\DragonAdventureIF\0000_00\DAIF_CharaData_0000_00.uasset'
)
foreach ($rel in $stockAssets) {
    $source = Join-Path $dirs.LegacyStaging $rel
    Assert-File $source "Extracted stock asset '$rel'"
    $dest = Join-Path $dirs.JsonStaging (([IO.Path]::GetFileNameWithoutExtension($source)) + '.json')
    Invoke-Checked $config.UAssetGUIPath @('tojson', $source, $dest, 'VER_UE5_1', $config.MappingName)
}

# --- Stage 3: Pure Domain JSON Transformation ---
Ensure-CleanDirectory $dirs.ModifiedJsonStaging
$transformScript = Join-Path $PSScriptRoot 'Transform-CompleteStoryAssets.ps1'
Assert-File $transformScript 'Transform-CompleteStoryAssets.ps1'

& $transformScript `
    -RegistryJson (Join-Path $dirs.JsonStaging 'DragonAdventureIFData.json') `
    -GokuJson (Join-Path $dirs.JsonStaging 'DAIF_CharaData_0000_00.json') `
    -ChartRegistryJson (Join-Path $dirs.JsonStaging 'DragonAdventureIFChartData.json') `
    -OutputDirectory $dirs.ModifiedJsonStaging

# --- Stage 4: Compile Modified JSON Back to UAsset ---
Ensure-CleanDirectory $dirs.ContainerStaging
$outputs = @(
    @{ Json='DragonAdventureIFData.modified.json'; Relative='SparkingZERO\Content\SS\Blueprints\DragonAdventureIFData.uasset' },
    @{ Json='DragonAdventureIFChartData.modified.json'; Relative='SparkingZERO\Content\SS\Blueprints\DragonAdventureIFChartData.uasset' },
    @{ Json='DAIF_CharaData_CompleteStory.json'; Relative='SparkingZERO\Content\SS\MasterDataAsset\DragonAdventureIF\CompleteStory\DAIF_CharaData_CompleteStory.uasset' }
)
foreach ($item in $outputs) {
    $src = Join-Path $dirs.ModifiedJsonStaging $item.Json
    $dest = Join-Path $dirs.ContainerStaging $item.Relative
    Assert-File $src "Modified JSON '$($item.Json)'"
    $null = New-Item -ItemType Directory -Force -Path (Split-Path -Parent $dest)
    Invoke-Checked $config.UAssetGUIPath @('fromjson', $src, $dest, $config.MappingName)
}
$scriptObjects = Join-Path $dirs.LegacyStaging 'scriptobjects.bin'
Assert-File $scriptObjects 'scriptobjects.bin from legacy extraction'
Copy-Item -LiteralPath $scriptObjects -Destination (Join-Path $dirs.ContainerStaging 'scriptobjects.bin') -Force

# --- Stage 5: Pack IoStore Container & Verify ---
if (-not (Test-Path -LiteralPath $dirs.Dist)) {
    $null = New-Item -ItemType Directory -Force -Path $dirs.Dist
}
$utocPath = Join-Path $dirs.Dist ($ContainerName + '.utoc')
Invoke-Checked $config.RetocPath @('to-zen', '--version', 'UE5_1', $dirs.ContainerStaging, $utocPath)

foreach ($ext in '.pak', '.utoc', '.ucas') {
    $cPath = Join-Path $dirs.Dist ($ContainerName + $ext)
    Assert-File $cPath "Container $ext file"
    if ((Get-Item -LiteralPath $cPath).Length -eq 0) { throw "Generated container $ext is empty: $cPath" }
}
Invoke-Checked $config.RetocPath @('verify', $utocPath)

# --- Stage 6: Package Unverum-Ready Archive ---
if ($PackageRelease) {
    $zipPath = Join-Path $dirs.Dist 'CompleteStory-v0.3-Unverum.zip'
    if (Test-Path -LiteralPath $zipPath) { Remove-Item -LiteralPath $zipPath -Force }
    $containerFiles = @('.pak', '.utoc', '.ucas') | ForEach-Object { Join-Path $dirs.Dist ($ContainerName + $_) }
    Compress-Archive -Path $containerFiles -DestinationPath $zipPath -CompressionLevel Optimal
    Assert-File $zipPath 'Unverum release archive'
    Write-Output "Complete Story build successful: $zipPath"
}

# --- Optional: Deploy directly to local game installation ---
if ($Deploy) {
    Write-Output "Deploying Complete Story build to local game installation..."
    $deployScript = Join-Path $PSScriptRoot 'Deploy-DevelopmentBuild.ps1'
    Assert-File $deployScript 'Deploy-DevelopmentBuild.ps1'
    & $deployScript -Install -BuildDirectory $dirs.Dist -ConfigPath $ConfigPath
}
