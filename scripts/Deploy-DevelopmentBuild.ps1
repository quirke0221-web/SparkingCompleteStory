<#
.SYNOPSIS
    Local developer convenience utility for staging and unstaging Complete Story in Sparking ZERO.
.DESCRIPTION
    Supports -Install and -Uninstall switches for local testing.
    End-user mod players should always install via Unverum mod manager.
#>
[CmdletBinding(SupportsShouldProcess)]
param(
    [Parameter(ParameterSetName = 'Install', Mandatory = $true)]
    [switch]$Install,

    [Parameter(ParameterSetName = 'Uninstall', Mandatory = $true)]
    [switch]$Uninstall,

    [Parameter(ParameterSetName = 'Install')]
    [string]$BuildDirectory,

    [string]$ConfigPath = (Join-Path $PSScriptRoot '..\config\project.local.psd1'),
    [string]$BackupRoot = (Join-Path $PSScriptRoot '..\local-handoff\deploy-backups')
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

. (Join-Path $PSScriptRoot 'Common.ps1')
$config = Get-ProjectConfiguration $ConfigPath
$dirs   = Get-PipelineDirectories

if (-not $BuildDirectory) { $BuildDirectory = $dirs.Dist }
$target = Join-Path $config.GameRoot 'SparkingZERO\Content\Paks\~mods\CompleteStory'
$files  = @('CompleteStory_P.pak', 'CompleteStory_P.utoc', 'CompleteStory_P.ucas')

if ($Install) {
    foreach ($f in $files) { Assert-File (Join-Path $BuildDirectory $f) "Build container file '$f'" }
    $timestamp = (Get-Date -Format 'yyyyMMdd-HHmmss')
    $backup = Join-Path $BackupRoot "install-$timestamp"

    if ($PSCmdlet.ShouldProcess($target, 'Backup existing CompleteStory mod and deploy new build')) {
        if (Test-Path -LiteralPath $target) {
            $null = New-Item -ItemType Directory -Force -Path $backup
            Copy-Item -LiteralPath $target -Destination $backup -Recurse
        }
        $null = New-Item -ItemType Directory -Force -Path $target
        foreach ($f in $files) {
            Copy-Item -LiteralPath (Join-Path $BuildDirectory $f) -Destination (Join-Path $target $f) -Force
        }
        Write-Output "Successfully installed development build to: $target"
    }
}
elseif ($Uninstall) {
    if (Test-Path -LiteralPath $target) {
        $timestamp = (Get-Date -Format 'yyyyMMdd-HHmmss')
        $backup = Join-Path $BackupRoot "uninstall-$timestamp"
        if ($PSCmdlet.ShouldProcess($target, "Backup to '$backup' and remove CompleteStory from ~mods")) {
            $null = New-Item -ItemType Directory -Force -Path $backup
            Copy-Item -LiteralPath $target -Destination (Join-Path $backup 'CompleteStory') -Recurse
            $srcCount = (Get-ChildItem -LiteralPath $target -Recurse -File).Count
            $bakCount = (Get-ChildItem -LiteralPath (Join-Path $backup 'CompleteStory') -Recurse -File).Count
            if ($srcCount -ne $bakCount) { throw "Backup verification failed; aborting uninstallation." }
            Remove-Item -LiteralPath $target -Recurse -Force
            Write-Output "Successfully uninstalled CompleteStory from: $target (Backup: $backup)"
        }
    } else {
        Write-Output "CompleteStory is not currently installed at: $target"
    }
}
