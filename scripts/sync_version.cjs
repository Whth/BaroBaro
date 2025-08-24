// scripts/sync-version.js
const fs = require('fs');
const path = require('path');

// Project root directory
const PROJECT_ROOT = path.resolve(__dirname, '..');
const PACKAGE_JSON = path.join(PROJECT_ROOT, 'package.json');
const CARGO_TOML = path.join(PROJECT_ROOT, 'src-tauri', 'Cargo.toml');

// Read package.json
const pkg = JSON.parse(fs.readFileSync(PACKAGE_JSON, 'utf-8'));
const newVersion = pkg.version;

console.log(`Syncing version ${newVersion} from package.json to Cargo.toml`);

// Read Cargo.toml
let cargoToml = fs.readFileSync(CARGO_TOML, 'utf-8');

// Replace version field (supports spaces and quotes)
cargoToml = cargoToml.replace(
    /^(version\s*=\s*)".*"/m,
    `$1"${newVersion}"`
);

// Write back
fs.writeFileSync(CARGO_TOML, cargoToml, 'utf-8');

console.log('✅ Cargo.toml updated successfully!');