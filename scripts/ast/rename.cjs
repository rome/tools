/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

require('../_setup.cjs');

const path = require('path');
const fs = require('fs');

const fromType = process.argv[2];
const toType = process.argv[3];
if (fromType === undefined || toType === undefined) {
  console.error('node rename-ast-type.js [from] [to]');
  process.exit(1);
}

function rename(src, dest) {
  fs.mkdirSync(path.dirname(dest), {recursive: true});
  console.log(path.relative(process.cwd(), src), '->', path.relative(
    process.cwd(),
    dest,
  ));
  fs.renameSync(src, dest);
}

const {
  generatorsFolder,
  analysisFolder,
  astFolder,
} = require('../_constants.cjs');

rename(path.join(generatorsFolder, `${fromType}.ts`), path.join(
  generatorsFolder,
  `${toType}ts`,
));
rename(path.join(analysisFolder, `${fromType}ts`), path.join(
  analysisFolder,
  `${toType}ts`,
));
rename(path.join(astFolder, `${fromType}.ts`), path.join(
  astFolder,
  `${toType}.ts`,
));

require('./update.cjs');
