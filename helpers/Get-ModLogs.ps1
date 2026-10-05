<#
.SYNOPSIS
    Diagnostic log inspector for Dragon Ball: Sparking! ZERO mod triage.
.DESCRIPTION
    Tails and inspects RE-UE4SS runtime logs, Unreal Engine 5 game logs,
    and recent engine crash dumps to enable autonomous AI agents and contributors
    to diagnose in-game crashes and script errors without manual user triage.
.PARAMETER TailLines
    Number of lines to read from the end of each log file (default: 50).
.PARAMETER ErrorsOnly
    Filter log output strictly for error, fatal, exception, or failure patterns.
.PARAMETER ConfigPath
    Path to project.local.psd1 configuration file.
#>
[CmdletBinding()]
param(
    [int]$TailLines = 50,
    [switch]$ErrorsOnly,
    [string]$ConfigPath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

if (-not $ConfigPath) {
    $ConfigPath = Join-Path $PSScriptRoot '..\config\project.local.psd1'
}

. (Join-Path $PSScriptRoot 'Common.ps1')

function Format-LogSection([string]$Title, [string]$FilePath) {
    Write-Output "================================================================================"
    Write-Output "[$Title] $FilePath"
    if (-not (Test-Path -LiteralPath $FilePath -PathType Leaf)) {
        Write-Output "Status: File not found (no logs recorded yet)."
        Write-Output ""
        return
    }

    $item = Get-Item -LiteralPath $FilePath
    Write-Output "Last Modified: $($item.LastWriteTime.ToString('yyyy-MM-dd HH:mm:ss')) | Size: $([math]::Round($item.Length / 1KB, 2)) KB"
    Write-Output "--------------------------------------------------------------------------------"

    $lines = Get-Content -LiteralPath $FilePath -Tail $TailLines -ErrorAction SilentlyContinue
    if (-not $lines) {
        Write-Output "(Log file is empty)"
    }
    elseif ($ErrorsOnly) {
        $errorPattern = '(?i)(error|fatal|exception|0xC0000005|access violation|failed to|assertion)'
        $matching = $lines | Where-Object { $_ -match $errorPattern }
        if ($matching) {
            $matching | ForEach-Object { Write-Output $_ }
        } else {
            Write-Output "No error patterns found in the last $TailLines lines."
        }
    }
    else {
        $lines | ForEach-Object { Write-Output $_ }
    }
    Write-Output ""
}

# Resolve game root from config if available
$gameRoot = $null
if (Test-Path -LiteralPath $ConfigPath -PathType Leaf) {
    try {
        $cfg = Get-ProjectConfiguration $ConfigPath
        $gameRoot = $cfg.GameRoot
    } catch {
        Write-Warning "Could not read GameRoot from '$ConfigPath': $($_.Exception.Message)"
    }
}

# 1. RE-UE4SS Log
if ($gameRoot) {
    $ue4ssLog = Join-Path $gameRoot 'SparkingZERO\Binaries\Win64\ue4ss.log'
    Format-LogSection 'RE-UE4SS Runtime Log' $ue4ssLog
} else {
    Write-Output "[RE-UE4SS Runtime Log] Skipped: GameRoot not configured in project.local.psd1."
    Write-Output ""
}

# 2. Engine Game Log
$localAppData = [Environment]::GetFolderPath('LocalApplicationData')
$engineLog = Join-Path $localAppData 'SparkingZERO\Saved\Logs\SparkingZERO.log'
Format-LogSection 'Unreal Engine 5 Game Log' $engineLog

# 3. Crash Dumps
Write-Output "================================================================================"
Write-Output "[Crash Dumps] $localAppData\SparkingZERO\Saved\Crashes"
$crashesDir = Join-Path $localAppData 'SparkingZERO\Saved\Crashes'
if (Test-Path -LiteralPath $crashesDir -PathType Container) {
    $recentCrashes = Get-ChildItem -LiteralPath $crashesDir -Directory |
        Sort-Object LastWriteTime -Descending |
        Select-Object -First 3

    if ($recentCrashes) {
        Write-Output "Found recent crash reports:"
        foreach ($c in $recentCrashes) {
            Write-Output " - $($c.Name) (Time: $($c.LastWriteTime.ToString('yyyy-MM-dd HH:mm:ss')))"
            $crashFiles = Get-ChildItem -LiteralPath $c.FullName -File
            foreach ($f in $crashFiles) {
                Write-Output "     * $($f.Name) ($([math]::Round($f.Length / 1KB, 1)) KB)"
            }
        }
    } else {
        Write-Output "No crash reports found in directory."
    }
} else {
    Write-Output "Status: No crashes directory found (no engine crashes recorded)."
}
Write-Output "================================================================================"
