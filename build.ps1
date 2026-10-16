# build.ps1 : compile le plugin et génère le .rgplugin
param(
    [switch]$Install   # -Install : installe aussi le plugin via rustgis-cli
)

$ErrorActionPreference = 'Stop'
$target = 'wasm32-unknown-unknown'
Push-Location $PSScriptRoot

try {
    # 1. Lire id et version depuis plugin.json
    $manifest = Get-Content plugin.json -Raw | ConvertFrom-Json
    $id = $manifest.id
    $version = $manifest.version

    # 2. Lire le nom du crate depuis Cargo (les tirets deviennent des underscores)
    $meta = cargo metadata --no-deps --format-version 1 | ConvertFrom-Json
    $crate = $meta.packages[0]
    $wasmName = ($crate.name -replace '-', '_') + '.wasm'

    if ($crate.version -ne $version) {
        Write-Warning "Versions différentes : Cargo.toml=$($crate.version) / plugin.json=$version"
    }

    # 3. Installer la cible wasm si elle manque
    if (-not (rustup target list --installed | Select-String -SimpleMatch $target)) {
        rustup target add $target
    }

    # 4. Compilation
    Write-Host "==> cargo build --release --target $target" -ForegroundColor Cyan
    cargo build --release --target $target
    if ($LASTEXITCODE -ne 0) { throw "Échec de la compilation" }

    $wasm = Join-Path $meta.target_directory "$target\release\$wasmName"
    if (-not (Test-Path $wasm)) { throw "WASM introuvable : $wasm" }

    # 5. Recréer pkg/ à neuf
    $pkg = Join-Path $PSScriptRoot 'pkg'
    if (Test-Path $pkg) { Remove-Item $pkg -Recurse -Force }
    New-Item -ItemType Directory $pkg | Out-Null
    Copy-Item plugin.json $pkg
    Copy-Item $wasm (Join-Path $pkg 'plugin.wasm')

    # 6. Supprimer les anciens .rgplugin de ce plugin (toutes versions)
    Get-ChildItem -Filter "$id-*.rgplugin" | Remove-Item -Force

    # 7. Créer l'archive directement en .rgplugin (sans passer par un .zip à renommer)
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $out = Join-Path $PSScriptRoot "$id-$version.rgplugin"
    [System.IO.Compression.ZipFile]::CreateFromDirectory($pkg, $out)

    $size = [math]::Round((Get-Item $out).Length / 1KB, 1)
    Write-Host "==> $out ($size Ko)" -ForegroundColor Green

    # 8. Installation (optionnelle)
    if ($Install) {
        rustgis-cli plugins install $out
        if ($LASTEXITCODE -ne 0) { throw "Échec de l'installation" }
    }
}
finally {
    Pop-Location
}
