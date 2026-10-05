param([string]$RepositoryRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path)

$ErrorActionPreference = 'Stop'
$tracked = @{}; git -C $RepositoryRoot ls-files | ForEach-Object { $tracked[$_.Replace('/','\')] = $true }
$rows = foreach ($file in Get-ChildItem -LiteralPath $RepositoryRoot -Recurse -File -Force | Where-Object FullName -notmatch '\\.git\\') {
    $relative = $file.FullName.Substring($RepositoryRoot.Length + 1)
    $category = if ($tracked.ContainsKey($relative) -or $relative -match '^(START_HERE\.md|docs\\|evidence\\|config\\project\.local\.example|scripts\\)') {
        'Tracked on GitHub'
    } elseif ($relative -match '(^|\\)(backup save|SaveGame)(\\|$)|backup save\.zip|steam_autocloud|MainGameSaveData|SystemSaveData') {
        'Sensitive/private material'
    } elseif ($relative -match '^tools\\|\.(uasset|uexp|ubulk|umap|usmap|dll|exe)$|^(global|CompleteStory_P)\.(pak|utoc|ucas)$') {
        'Third-party or copyrighted game material'
    } elseif ($relative -match '^dist\\|^build\\|^decode_input\\|\.(pak|utoc|ucas|zip)$') {
        'Generated/rebuildable locally'
    } elseif ($relative -match '^work\\') {
        'Third-party or copyrighted game material'
    } elseif ($relative -match '^mods\\|^runtime\\CompleteStory\\') {
        'Superseded or obsolete'
    } else {
        'Missing and safe to publish'
    }
    [pscustomobject]@{ Path=$relative.Replace('\','/'); Category=$category; Bytes=$file.Length }
}

$lines = @('# File Audit','',"Generated: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss K')",'',
    'This inventory preserves local files and classifies them for the public repository. A classification of excluded does not mean deleted. Save identifiers are not reproduced in file rows; save folders are summarized below.','')
$safeRows = $rows | Where-Object { $_.Path -notmatch '(^|/)backup save(/|$)|test_backups/.*/SaveGame/' } | Sort-Object Path
foreach ($group in $safeRows | Group-Object Category | Sort-Object Name) {
    $lines += "## $($group.Name)"
    $lines += ''
    $lines += '| Path | Bytes |'
    $lines += '|---|---:|'
    foreach ($row in $group.Group) { $lines += "| ``$($row.Path)`` | $($row.Bytes) |" }
    $lines += ''
}
$private = $rows | Where-Object { $_.Category -eq 'Sensitive/private material' }
$lines += '## Redacted private groups'
$lines += ''
$lines += "- Save/backup files: $($private.Count) files, $((($private | Measure-Object Bytes -Sum).Sum)) bytes. Paths containing the Steam account identifier are deliberately omitted."
$lines += '- These files remain local and are ignored by Git.'
$lines += ''
$lines += '## Audit interpretation'
$lines += ''
$lines += '- Generated game-derived JSON is excluded even when human-readable; it serializes copyrighted assets. Small derived summaries are under `evidence/assets/`.'
$lines += '- Third-party tools are named and versioned in documentation but not vendored.'
$lines += '- Historical project-authored Lua is tracked under `archive/legacy/`; duplicate installed/test copies remain excluded.'

$destination = Join-Path $RepositoryRoot 'docs\FILE_AUDIT.md'
New-Item -ItemType Directory -Force -Path (Split-Path -Parent $destination) | Out-Null
[IO.File]::WriteAllLines($destination, $lines, [Text.UTF8Encoding]::new($false))
Write-Output $destination
