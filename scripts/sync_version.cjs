// scripts/sync-version.js
const fs = require('fs');
const path = require('path');

// Project root directory
const PROJECT_ROOT = path.resolve(__dirname, '..');
const PACKAGE_JSON = path.join(PROJECT_ROOT, 'package.json');
const CARGO_TOML = path.join(PROJECT_ROOT, 'src-tauri', 'Cargo.toml');
const TAURI_CONF = path.join(PROJECT_ROOT, 'src-tauri', 'tauri.conf.json');

// Read package.json
const pkg = JSON.parse(fs.readFileSync(PACKAGE_JSON, 'utf-8'));
const newVersion = pkg.version;

console.log(`Syncing version ${newVersion} from package.json to Cargo.toml and tauri.conf.json`);

// === Update Cargo.toml ===
let cargoToml = fs.readFileSync(CARGO_TOML, 'utf-8');
cargoToml = cargoToml.replace(
    /^(version\s*=\s*)".*"/m,
    `$1"${newVersion}"`
);
fs.writeFileSync(CARGO_TOML, cargoToml, 'utf-8');
console.log('✅ Cargo.toml updated successfully!');

// === Update tauri.conf.json ===
const tauriConf = JSON.parse(fs.readFileSync(TAURI_CONF, 'utf-8'));

if (tauriConf.version !== newVersion) {
    tauriConf.version = newVersion;
    // Pretty-print with 2-space indentation
    fs.writeFileSync(TAURI_CONF, JSON.stringify(tauriConf, null, 2) + '\n', 'utf-8');
    console.log('✅ tauri.conf.json updated successfully!');
} else {
    console.log('🟨 tauri.conf.json already up to date.');
}