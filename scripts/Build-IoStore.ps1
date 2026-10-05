param(
    [Parameter(Mandatory)][string]$StageDirectory,
    [Parameter(Mandatory)][string]$OutputDirectory,
    [string]$ContainerName = 'CompleteStory_P',
    [string]$ConfigPath = (Join-Path $PSScriptRoot '..\config\project.local.psd1')
)

. (Join-Path $PSScriptRoot 'Common.ps1')
$config = Get-ProjectConfiguration $ConfigPath
Assert-File $config.RetocPath 'retoc'
$scriptObjects = Join-Path $StageDirectory 'scriptobjects.bin'
Assert-File $scriptObjects 'scriptobjects.bin copied by the targeted extraction step'

New-Item -ItemType Directory -Force -Path $OutputDirectory | Out-Null
$utoc = Join-Path $OutputDirectory ($ContainerName + '.utoc')
Invoke-Checked $config.RetocPath @('to-zen', '--version', 'UE5_1', $StageDirectory, $utoc)

foreach ($extension in '.pak', '.utoc', '.ucas') {
    Assert-File (Join-Path $OutputDirectory ($ContainerName + $extension)) "Generated $extension container"
}
