/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

const path = require('path');

const root = path.join(__dirname, '..', '..');
const packages = path.join(root, 'packages', '@romejs');
const generatorsFolder = path.join(packages, 'js-generator', 'generators');
const analysisFolder = path.join(packages, 'js-analysis', 'evaluators');
const astFolder = path.join(packages, 'js-ast');

module.exports = {
  root,
  packages,
  generatorsFolder,
  analysisFolder,
  astFolder,
};
