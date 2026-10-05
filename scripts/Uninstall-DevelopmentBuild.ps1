[CmdletBinding(SupportsShouldProcess)]
param([string]$ConfigPath = (Join-Path $PSScriptRoot '..\config\project.local.psd1'))

. (Join-Path $PSScriptRoot 'Common.ps1')
$config = Get-ProjectConfiguration $ConfigPath
$target = Join-Path $config.GameRoot 'SparkingZERO\Content\Paks\~mods\CompleteStory'
if (Test-Path -LiteralPath $target) {
    if ($PSCmdlet.ShouldProcess($target, 'Remove only the CompleteStory development-mod directory')) {
        Remove-Item -LiteralPath $target -Recurse -Force
    }
}
