/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

const path = require('path');
const fs = require('fs');

const filename = process.argv[2];
if (filename === undefined) {
  console.error('node remove-ast-type.js [category]/[nodeType]');
  process.exit(1);
}

const {
  generatorsFolder,
  analysisFolder,
  astFolder,
} = require('../_constants.cjs');

fs.unlinkSync(path.join(generatorsFolder, filename + '.ts'));
fs.unlinkSync(path.join(analysisFolder, filename + '.ts'));
fs.unlinkSync(path.join(astFolder, filename + '.ts'));

require('./update.cjs');
