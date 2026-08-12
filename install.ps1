# runo.tokyo installer for Windows.
# ソースからビルドしてインストールする(ビルド済みバイナリの署名/配布
# パイプラインは今回未整備、他のRS-系リポジトリと同じ暫定方式)。
# 必要なもの: git, Rustツールチェーン(無ければwinget経由で自動導入)。

$ErrorActionPreference = "Stop"

$InstallDir = if ($env:RUNO_TOKYO_HOME) { $env:RUNO_TOKYO_HOME } else { Join-Path $env:LOCALAPPDATA "runo-tokyo" }
$BinDir = Join-Path $InstallDir "bin"
$RepoUrl = "https://github.com/aon-co-jp/runo.tokyo.git"

Write-Host "== runo.tokyo installer (Windows) =="

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Rust toolchain not found."
    if (Get-Command winget -ErrorAction SilentlyContinue) {
        Write-Host "Installing via winget (Rustlang.Rustup)..."
        winget install --id Rustlang.Rustup -e --source winget
    } else {
        Write-Host "Please install Rust manually from https://rustup.rs and re-run this script."
        exit 1
    }
}

if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    Write-Host "git is required but was not found on PATH. Install Git for Windows first: https://git-scm.com/download/win"
    exit 1
}

if (-not (Test-Path $InstallDir)) {
    git clone $RepoUrl $InstallDir
} else {
    Write-Host "Existing install found at $InstallDir, pulling latest..."
    git -C $InstallDir pull --ff-only
}

Set-Location $InstallDir
cargo build --release

New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
Copy-Item (Join-Path $InstallDir "target\release\runo-tokyo.exe") (Join-Path $BinDir "runo-tokyo.exe") -Force

Write-Host ""
Write-Host "Installed: $(Join-Path $BinDir 'runo-tokyo.exe')"
if (($env:Path -split ";") -notcontains $BinDir) {
    Write-Host "Note: $BinDir is not on your PATH. Add it, e.g.:"
    Write-Host "  [Environment]::SetEnvironmentVariable('Path', `$env:Path + ';$BinDir', 'User')"
}

Write-Host ""
Write-Host "Usage:"
Write-Host "  runo-tokyo.exe   # 実行してWebサーバーを起動"
