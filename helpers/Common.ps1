Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Get-ProjectConfiguration {
    param([string]$ConfigPath = (Join-Path $PSScriptRoot '..\config\project.local.psd1'))

    if (-not (Test-Path -LiteralPath $ConfigPath -PathType Leaf)) {
        throw "Missing local configuration: $ConfigPath. Copy config/project.local.example.psd1 to project.local.psd1 first."
    }

    $config = Import-PowerShellDataFile -LiteralPath $ConfigPath
    foreach ($name in 'GameRoot', 'RetocPath', 'UAssetGUIPath', 'MappingName', 'MappingPath', 'AesKeyEnvironmentVariable') {
        if (-not $config.ContainsKey($name) -or [string]::IsNullOrWhiteSpace([string]$config[$name])) {
            throw "Configuration value '$name' is required."
        }
    }
    return $config
}

function Get-PipelineDirectories {
    $projectRoot = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
    return @{
        ProjectRoot          = $projectRoot
        Staging              = Join-Path $projectRoot 'staging'
        LegacyStaging        = Join-Path $projectRoot 'staging\legacy'
        JsonStaging          = Join-Path $projectRoot 'staging\json'
        ModifiedJsonStaging  = Join-Path $projectRoot 'staging\modified-json'
        ContainerStaging     = Join-Path $projectRoot 'staging\container'
        Dist                 = Join-Path $projectRoot 'dist'
        RuntimeSource        = Join-Path $projectRoot 'CompleteStory'
    }
}

function Assert-File([string]$Path, [string]$Description) {
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "$Description was not found: $Path"
    }
}

function Ensure-CleanDirectory([string]$Path) {
    if (Test-Path -LiteralPath $Path) {
        Remove-Item -LiteralPath $Path -Recurse -Force
    }
    $null = New-Item -Path $Path -ItemType Directory -Force
}

function Invoke-Checked {
    param([string]$FilePath, [string[]]$Arguments)
    $global:LASTEXITCODE = 0
    & $FilePath @Arguments
    if ($LASTEXITCODE -ne 0) {
        # Never echo arguments here: retoc calls can include the owner's AES key.
        throw "Command failed with exit code ${LASTEXITCODE}: $FilePath (arguments redacted)"
    }
}

function Wait-FileReady {
    param(
        [string]$Path,
        [string]$Description,
        [int]$TimeoutSeconds = 30
    )

    $deadline = [DateTime]::UtcNow.AddSeconds($TimeoutSeconds)
    $stableLength = -1L
    $stableChecks = 0
    while ([DateTime]::UtcNow -lt $deadline) {
        if (Test-Path -LiteralPath $Path -PathType Leaf) {
            $length = (Get-Item -LiteralPath $Path).Length
            if ($length -gt 0 -and $length -eq $stableLength) {
                $stableChecks++
                if ($stableChecks -ge 3) { return }
            } else {
                $stableLength = $length
                $stableChecks = 1
            }
        }
        Start-Sleep -Milliseconds 100
    }
    throw "Timed out waiting for $Description`: $Path"
}
