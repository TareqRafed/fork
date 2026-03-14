$ErrorActionPreference = "Stop"

$Repo = "TareqRafed/fork"
$Bin = "fork"
$Target = "x86_64-pc-windows-msvc"
$InstallDir = if ($env:INSTALL_DIR) { $env:INSTALL_DIR } else { "$env:LOCALAPPDATA\Programs\fork" }

# Resolve version
$Version = $env:VERSION
if (-not $Version) {
    $Release = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
    $Version = $Release.tag_name
}

if (-not $Version) {
    Write-Error "Could not determine latest version. Set `$env:VERSION manually."
    exit 1
}

$Filename = "$Bin-$Version-$Target.zip"
$Url = "https://github.com/$Repo/releases/download/$Version/$Filename"

Write-Host "Installing $Bin $Version..."

$Tmp = New-TemporaryFile | ForEach-Object { $_.DirectoryName + "\" + $_.BaseName }
New-Item -ItemType Directory -Path $Tmp | Out-Null

try {
    $ZipPath = "$Tmp\$Filename"
    Invoke-WebRequest $Url -OutFile $ZipPath
    Expand-Archive $ZipPath -DestinationPath $Tmp

    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir | Out-Null
    }

    Copy-Item "$Tmp\$Bin.exe" "$InstallDir\$Bin.exe" -Force
} finally {
    Remove-Item $Tmp -Recurse -Force -ErrorAction SilentlyContinue
}

# Add to PATH if not already present
$CurrentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($CurrentPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$CurrentPath;$InstallDir", "User")
    Write-Host "Added $InstallDir to PATH (restart your terminal to apply)"
}

Write-Host "$Bin $Version installed to $InstallDir\$Bin.exe"
