const os = require("os");

const platform = `${os.platform()}-${os.arch()}`;
const packageName = `@jsrsc/mgchecker-${platform}`;

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
exports.init_sensitive_set = checker.init_sensitive_set;

/**
 * function
 * params String
 * return Boolean
 */
exports.has_sensitive_word = checker.has_sensitive_word;
