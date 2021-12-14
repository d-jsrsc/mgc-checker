const fs = require("fs");
const { execSync } = require("child_process");
let distDir = "platforms/mgchecker-darwin-arm64";
let src = "target/release/libmgchecker.dylib";
let dist = `${distDir}/index.node`;

fs.copyFileSync(src, dist);
// const r = execSync(`ls`);
// console.log('---', r)
try {
  execSync(`cd ${distDir} && npm publish --access public`);
} catch (error) {
  console.log(error);
}
