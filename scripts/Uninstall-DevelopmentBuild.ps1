[CmdletBinding(SupportsShouldProcess)]
param(
    [string]$ConfigPath = (Join-Path $PSScriptRoot '..\config\project.local.psd1'),
    [string]$BackupRoot = (Join-Path $PSScriptRoot '..\local-handoff\uninstall-backups')
)

. (Join-Path $PSScriptRoot 'Common.ps1')
$config = Get-ProjectConfiguration $ConfigPath
$target = Join-Path $config.GameRoot 'SparkingZERO\Content\Paks\~mods\CompleteStory'
if (Test-Path -LiteralPath $target) {
    $backup = Join-Path $BackupRoot (Get-Date -Format 'yyyyMMdd-HHmmss')
    if ($PSCmdlet.ShouldProcess($target, "Back up to '$backup', then remove only the CompleteStory development-mod directory")) {
        New-Item -ItemType Directory -Force -Path $backup | Out-Null
        Copy-Item -LiteralPath $target -Destination (Join-Path $backup 'CompleteStory') -Recurse
        $sourceFiles = Get-ChildItem -LiteralPath $target -Recurse -File
        $backupFiles = Get-ChildItem -LiteralPath (Join-Path $backup 'CompleteStory') -Recurse -File
        if ($sourceFiles.Count -ne $backupFiles.Count) {
            throw "Backup verification failed; installation was not removed. Backup: $backup"
        }
        Remove-Item -LiteralPath $target -Recurse -Force
        "Removed $target. Restorable backup: $backup"
    }
}
