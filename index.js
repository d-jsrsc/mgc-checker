const os = require("os");

const platform = `mgchecker-${os.platform()}-${os.arch()}`;
const packageName = `@jsrsc/${platform}`;

if (!require("./package.json").optionalDependencies[packageName]) {
  throw "Unsupported platform/architecture: " + platform;
}
const checker = require(packageName);

/**
 * function
 * return string
 */
exports.hello = checker.hello;

/**
 * function
 * params Array<String>
 * return undefind
 */
exports.initSensitiveSet = checker.initSensitiveSet;

/**
 * function
 * params String
 * params callback(err, Boolean)
 */
exports.hasSensitiveWord = checker.hasSensitiveWord;

/**
 * function
 * params String
 * return Boolean
 */
exports.hasSensitiveWordSync = checker.hasSensitiveWordSync;
