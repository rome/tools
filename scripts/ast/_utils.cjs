/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

const path = require('path');
const fs = require('fs');

exports.write = function write(loc, content) {
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
