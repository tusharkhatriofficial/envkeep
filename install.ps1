# envkeep Windows installer
# Run this in PowerShell (not CMD):
#   irm https://raw.githubusercontent.com/tusharkhatriofficial/envkeep/main/install.ps1 | iex

$ErrorActionPreference = "Stop"
$Repo = "tusharkhatriofficial/envkeep"

Write-Host "Fetching latest envkeep release..."
$Release = (Invoke-RestMethod "https://api.github.com/repos/$Repo/releases") | Select-Object -First 1
$Tag = $Release.tag_name
$Asset = $Release.assets | Where-Object { $_.name -like "*windows*msvc*.zip" } | Select-Object -First 1

if (-not $Asset) {
    Write-Error "Could not find a Windows binary in release $Tag. Check https://github.com/$Repo/releases"
    exit 1
}

$ZipPath = "$env:TEMP\envkeep.zip"
$InstDir = "$env:USERPROFILE\.local\bin"
$ExePath = "$InstDir\envkeep.exe"

Write-Host "Downloading envkeep $Tag for Windows..."
Invoke-WebRequest -Uri $Asset.browser_download_url -OutFile $ZipPath -UseBasicParsing

New-Item -ItemType Directory -Force -Path $InstDir | Out-Null
Expand-Archive -Path $ZipPath -DestinationPath $InstDir -Force
Remove-Item $ZipPath

# Add install dir to user PATH if not already present
$CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($CurrentPath -notlike "*$InstDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$CurrentPath;$InstDir", "User")
    Write-Host ""
    Write-Host "envkeep $Tag installed to $ExePath"
    Write-Host ""
    Write-Host "PATH updated. Close and reopen your terminal, then run:"
    Write-Host "  envkeep --help"
} else {
    Write-Host ""
    Write-Host "envkeep $Tag installed to $ExePath"
    Write-Host ""
    & $ExePath --help
}
