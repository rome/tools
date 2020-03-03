/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

const path = require('path');

exports.root = path.join(__dirname, '..');
exports.packages = path.join(exports.root, 'packages', '@romejs');
exports.generatorsFolder = path.join(
  exports.packages,
  'js-generator',
  'generators',
);
exports.analysisFolder = path.join(
  exports.packages,
  'js-analysis',
  'evaluators',
);
exports.astFolder = path.join(exports.packages, 'js-ast');
exports.argv = process.argv.slice(2);
