param(
    [Parameter(Mandatory)][string]$ContainerDirectory,
    [string]$ContainerName = 'CompleteStory_P',
    [string]$ConfigPath = (Join-Path $PSScriptRoot '..\config\project.local.psd1')
)

. (Join-Path $PSScriptRoot 'Common.ps1')
$config = Get-ProjectConfiguration $ConfigPath
Assert-File $config.RetocPath 'retoc'
$utoc = Join-Path $ContainerDirectory ($ContainerName + '.utoc')
foreach ($extension in '.pak', '.utoc', '.ucas') {
    $path = Join-Path $ContainerDirectory ($ContainerName + $extension)
    Assert-File $path "Container $extension"
    if ((Get-Item -LiteralPath $path).Length -eq 0) { throw "Container is empty: $path" }
}
Invoke-Checked $config.RetocPath @('verify', $utoc)
Get-ChildItem -LiteralPath $ContainerDirectory -File | Where-Object Extension -in '.pak','.utoc','.ucas' |
    Get-FileHash -Algorithm SHA256 | Select-Object Path, Hash
