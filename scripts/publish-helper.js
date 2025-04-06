const fs = require('fs');
const oldPkg = require('../publish/package.json');

const flowFile = 'whisky_js.js.flow';
if (oldPkg.files.find(entry => entry === flowFile) == null) {
  oldPkg.files.push(flowFile);
}
if (oldPkg.name === 'whisky-js') {
  oldPkg.name = '@sidan-lab/' + oldPkg.name + process.argv.slice(2)[0];
}
if (process.argv.slice(2)[0] === '-browser' || process.argv.slice(2)[0] === '-asmjs') {
  // due to a bug in wasm-pack, this file is missing from browser builds
  const missingFile = 'whisky_js_bg.js';
  if (oldPkg.files.find(entry => entry === missingFile) == null) {
    oldPkg.files.push(missingFile);
  }
}
if (process.argv.slice(2)[0] === '-asmjs') {
  // need to replace WASM with ASM package 
  const missingFile = 'whisky_js_bg.wasm';
  oldPkg.files = [
    'whisky_js.asm.js',
    ...oldPkg.files.filter(file => file !== 'whisky_js_bg.wasm')
  ];
}

oldPkg.repository = {
  type: "git",
  url: "git+https://github.com/sidan-lab/whisky.git"
};
oldPkg.author = "sidan-lab";
oldPkg.license = "Apache-2.0";
console.log(oldPkg);
fs.writeFileSync('./publish/package.json', JSON.stringify(oldPkg, null, 2));
