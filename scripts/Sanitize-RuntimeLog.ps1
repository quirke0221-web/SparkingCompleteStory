param(
    [Parameter(Mandatory)][string]$InputPath,
    [Parameter(Mandatory)][string]$OutputPath
)

$ErrorActionPreference = 'Stop'
$text = Get-Content -LiteralPath $InputPath -Raw
$text = $text -replace '(?i)[A-Z]:\\SteamLibrary\\steamapps\\common\\DRAGON BALL Sparking! ZERO', '<GAME_ROOT>'
$text = $text -replace '(?i)C:\\Users\\[^\\\r\n]+', '<USER_PROFILE>'
$text = $text -replace '(?<!\d)7656119\d{10}(?!\d)', '<STEAM_ID>'
$text = $text -replace '(?i)0x[0-9a-f]{64}', '<REDACTED_256_BIT_VALUE>'
$parent = Split-Path -Parent $OutputPath
if ($parent) { New-Item -ItemType Directory -Force -Path $parent | Out-Null }
[IO.File]::WriteAllText($OutputPath, $text, [Text.UTF8Encoding]::new($false))
Write-Output $OutputPath
