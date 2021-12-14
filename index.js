const os = require("os");

const platform = `${os.platform()}-${os.arch()}`;
const packageName = `@jsrsc/mgchecker-${platform}`;

if (!require("./package.json").optionalDependencies[packageName]) {
  throw "Unsupported platform/architecture: " + platform;
}
module.exports = require(packageName);
