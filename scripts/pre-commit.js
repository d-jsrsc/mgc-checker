const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const { version: packageVersion } = require('../package.json');

const platformDir = path.join(path.resolve(__dirname, ".."), "platforms");
// console.log('prepublish', platformDir)
const dirs = fs.readdirSync(platformDir);
// console.log(dirs)
dirs.forEach((item) => {
    const itemPath = path.join(platformDir, item);
    const {version: subVersion} = require(path.join(itemPath, './package.json'));
    if (packageVersion != subVersion) {
        try {
            execSync(`cd ${itemPath} && npm version ${packageVersion} && git add .`);
        } catch (error) {
            process.exit(1);
        }
    }
})