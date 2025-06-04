const manifest_v2 = require('../versions/manifest_v2.json');
const manifest_v3 = require('../static/manifest.json');
const TOML = require('@iarna/toml');
const fs = require('fs');

const cargo = TOML.parse(fs.readFileSync('Cargo.toml', 'utf-8'));

const parseVersion = (version) => {
    const [major, minor, patch] = version.split('.');
    return {
        major: parseInt(major),
        minor: parseInt(minor),
        patch: parseInt(patch),
    };
};

const upgradeMajorVersion = (version) => {
    const { major } = parseVersion(version);
    return `${major + 1}.0.0`;
};

const upgradeMinorVersion = (version) => {
    const { major, minor } = parseVersion(version);
    return `${major}.${minor + 1}.0`;
};

const upgradePatchVersion = (version) => {
    const { major, minor, patch } = parseVersion(version);
    return `${major}.${minor}.${patch + 1}`;
};

const upgradeVersion = (version) => {
    const args = process.argv.slice(2);
    if (args.length === 0) {
        throw new Error('Please specify the type of version upgrade');
    }
    const type = args[0];
    switch (type) {
        case 'major':
            return upgradeMajorVersion(version);
        case 'minor':
            return upgradeMinorVersion(version);
        case 'patch':
            return upgradePatchVersion(version);
        default:
            return upgradePatchVersion(version);
    }
};

manifest_v2.version = upgradeVersion(manifest_v2.version);
manifest_v3.version = upgradeVersion(manifest_v3.version);
cargo.package.version = upgradeVersion(cargo.package.version);

fs.writeFileSync('versions/manifest_v2.json', JSON.stringify(manifest_v2, null, 2));
fs.writeFileSync('static/manifest.json', JSON.stringify(manifest_v3, null, 2));
fs.writeFileSync('Cargo.toml', TOML.stringify(cargo));