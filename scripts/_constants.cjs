/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

const path = require('path');
const os = require('os');

exports.devFolder = path.join(os.tmpdir(), 'rome-dev');
exports.root = path.join(__dirname, '..');
exports.packages = path.join(exports.root, 'packages', '@romejs');
exports.formatterFolder = path.join(
  exports.packages,
  'js-formatter',
  'builders',
);
exports.analysisFolder = path.join(
  exports.packages,
  'js-analysis',
  'evaluators',
);
exports.lintRulesFolder = path.join(
  exports.packages,
  'js-compiler',
  'lint',
  'rules',
);
exports.categoriesFile = path.join(
  exports.packages,
  'diagnostics',
  'categories.ts',
);
exports.descriptionsFile = path.join(
  exports.packages,
  'diagnostics',
  'descriptions.ts',
);
exports.astFolder = path.join(exports.packages, 'js-ast');
exports.argv = process.argv.slice(2);
