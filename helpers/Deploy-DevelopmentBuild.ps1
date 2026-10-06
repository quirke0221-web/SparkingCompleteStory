<#
.SYNOPSIS
    Local developer convenience utility for staging and unstaging Complete Story in Sparking ZERO.
.DESCRIPTION
    Supports -Install and -Uninstall switches for local testing.
    Stages both the IoStore container (~mods) and the RE-UE4SS runtime mod (Win64\Mods).
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
$pakTarget     = Join-Path $config.GameRoot 'SparkingZERO\Content\Paks\~mods\CompleteStory'
$runtimeTarget = Join-Path $config.GameRoot 'SparkingZERO\Binaries\Win64\Mods\CompleteStory'
$runtimeSource = Join-Path $PSScriptRoot '..\CompleteStory'
$modsList      = Join-Path $config.GameRoot 'SparkingZERO\Binaries\Win64\Mods\mods.txt'
$modEntryRegex = '^\s*CompleteStory\s*:\s*[01]\s*$'
$pakFiles      = @('CompleteStory_P.pak', 'CompleteStory_P.utoc', 'CompleteStory_P.ucas')

if ($Install) {
    foreach ($f in $pakFiles) { Assert-File (Join-Path $BuildDirectory $f) "Build container file '$f'" }
    Assert-File (Join-Path $runtimeSource 'scripts\main.lua') 'Runtime main.lua entrypoint'
    Assert-File (Join-Path $runtimeSource 'enabled.txt') 'Runtime enabled.txt flag'

    $timestamp = (Get-Date -Format 'yyyyMMdd-HHmmss')
    $backup    = Join-Path $BackupRoot "install-$timestamp"

    if ($PSCmdlet.ShouldProcess($config.GameRoot, 'Backup existing CompleteStory mod files and deploy new build')) {
        if ((Test-Path -LiteralPath $pakTarget) -or (Test-Path -LiteralPath $runtimeTarget) -or
            (Test-Path -LiteralPath $modsList)) {
            $null = New-Item -ItemType Directory -Force -Path $backup
            if (Test-Path -LiteralPath $pakTarget) {
                Copy-Item -LiteralPath $pakTarget -Destination (Join-Path $backup 'ContentMods') -Recurse
            }
            if (Test-Path -LiteralPath $runtimeTarget) {
                Copy-Item -LiteralPath $runtimeTarget -Destination (Join-Path $backup 'RuntimeMod') -Recurse
            }
            if (Test-Path -LiteralPath $modsList) {
                Copy-Item -LiteralPath $modsList -Destination (Join-Path $backup 'mods.txt')
            }
        }

        # Deploy container assets
        $null = New-Item -ItemType Directory -Force -Path $pakTarget
        foreach ($f in $pakFiles) {
            Copy-Item -LiteralPath (Join-Path $BuildDirectory $f) -Destination (Join-Path $pakTarget $f) -Force
        }

        # Deploy runtime mod
        $null = New-Item -ItemType Directory -Force -Path $runtimeTarget
        Copy-Item -Path (Join-Path $runtimeSource '*') -Destination $runtimeTarget -Recurse -Force

        $modLines = if (Test-Path -LiteralPath $modsList) { @(Get-Content -LiteralPath $modsList) } else { @() }
        $modLines = @($modLines | Where-Object { $_ -notmatch $modEntryRegex }) + 'CompleteStory : 1'
        [IO.File]::WriteAllLines($modsList, $modLines, [Text.UTF8Encoding]::new($false))

        Write-Output "Successfully deployed Complete Story container to: $pakTarget"
        Write-Output "Successfully deployed Complete Story runtime mod to: $runtimeTarget"
    }
}
elseif ($Uninstall) {
    $hasPak     = Test-Path -LiteralPath $pakTarget
    $hasRuntime = Test-Path -LiteralPath $runtimeTarget
    $hasModEntry = (Test-Path -LiteralPath $modsList) -and
        [bool](Select-String -LiteralPath $modsList -Pattern $modEntryRegex -Quiet)

    if ($hasPak -or $hasRuntime -or $hasModEntry) {
        $timestamp = (Get-Date -Format 'yyyyMMdd-HHmmss')
        $backup    = Join-Path $BackupRoot "uninstall-$timestamp"

        if ($PSCmdlet.ShouldProcess($config.GameRoot, "Backup and remove CompleteStory from game")) {
            $null = New-Item -ItemType Directory -Force -Path $backup
            if ($hasPak) {
                Copy-Item -LiteralPath $pakTarget -Destination (Join-Path $backup 'ContentMods') -Recurse
                Remove-Item -LiteralPath $pakTarget -Recurse -Force
            }
            if ($hasRuntime) {
                Copy-Item -LiteralPath $runtimeTarget -Destination (Join-Path $backup 'RuntimeMod') -Recurse
                Remove-Item -LiteralPath $runtimeTarget -Recurse -Force
            }
            if ($hasModEntry) {
                Copy-Item -LiteralPath $modsList -Destination (Join-Path $backup 'mods.txt')
                $modLines = @(Get-Content -LiteralPath $modsList) | Where-Object { $_ -notmatch $modEntryRegex }
                [IO.File]::WriteAllLines($modsList, $modLines, [Text.UTF8Encoding]::new($false))
            }
            Write-Output "Successfully uninstalled CompleteStory (Backup: $backup)"
        }
    } else {
        Write-Output "CompleteStory is not currently installed in: $($config.GameRoot)"
    }
}
