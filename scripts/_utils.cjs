/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

const child = require('child_process');
const path = require('path');
const fs = require('fs');

exports.unlink = function(loc) {
  if (!fs.existsSync(loc)) {
    return;
  }

  const stats = fs.lstatSync(loc);
  if (stats.isFile()) {
    fs.unlinkSync(loc);
  } else if (stats.isDirectory()) {
    for (const filename of fs.readdirSync(loc)) {
      exports.unlink(path.join(loc, filename));
    }
    fs.rmdirSync(loc);
  }
};

exports.exec = function(cmd, args) {
  const res = child.spawnSync(cmd, args, {
    stdio: 'inherit',
  });
  if (res.status !== 0) {
    process.exit(1);
  }
};

exports.execNode = function(args) {
  exports.exec(process.execPath, [...process.execArgv, ...args]);
};

exports.write = function(loc, content) {
  console.log('Wrote', loc);
  fs.mkdirSync(path.dirname(loc), {recursive: true});
  fs.writeFileSync(loc, content);
};

exports.getBuilderName = function(name) {
  const [startingCapitals] = name.match(/^([A-Z]+)/);

  if (startingCapitals.length === 1) {
    // Only one capital
    return name[0].toLowerCase() + name.slice(1);
  } else {
    // Take all and capitalize the first lowercase
    const rest = name.slice(startingCapitals.length - 1);
    return startingCapitals.slice(0, -1).toLowerCase() + rest;
  }
};
