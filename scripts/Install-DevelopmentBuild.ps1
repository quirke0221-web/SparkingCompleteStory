[CmdletBinding(SupportsShouldProcess)]
param(
    [Parameter(Mandatory)][string]$BuildDirectory,
    [string]$ConfigPath = (Join-Path $PSScriptRoot '..\config\project.local.psd1')
)

. (Join-Path $PSScriptRoot 'Common.ps1')
$config = Get-ProjectConfiguration $ConfigPath
$target = Join-Path $config.GameRoot 'SparkingZERO\Content\Paks\~mods\CompleteStory'
$backup = Join-Path $PSScriptRoot ('..\local-handoff\install-backups\' + (Get-Date -Format 'yyyyMMdd-HHmmss'))
$files = @('CompleteStory_P.pak','CompleteStory_P.utoc','CompleteStory_P.ucas')
foreach ($name in $files) { Assert-File (Join-Path $BuildDirectory $name) "Development build file '$name'" }

if ($PSCmdlet.ShouldProcess($target, 'Back up an existing CompleteStory install and install this build')) {
    if (Test-Path -LiteralPath $target) {
        New-Item -ItemType Directory -Force -Path $backup | Out-Null
        Copy-Item -LiteralPath $target -Destination $backup -Recurse
    }
    New-Item -ItemType Directory -Force -Path $target | Out-Null
    foreach ($name in $files) { Copy-Item -LiteralPath (Join-Path $BuildDirectory $name) -Destination (Join-Path $target $name) -Force }
    "Installed to $target. Existing files, if any, were copied to $backup."
}
