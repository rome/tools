//usr/bin/env node
/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

require('../_setup.cjs');

const path = require('path');
const fs = require('fs');

const {root, packages} = require('../_constants.cjs');
const {readGeneratedFile, write} = require('../_utils.cjs');

const runtimeModules = path.join(root, 'packages', '@romejs-runtime');
const runtimeIndex = path.join(
  packages,
  'core',
  'master',
  'fs',
  'runtime-modules.ts',
);

let runtimeIndexFile = readGeneratedFile(runtimeIndex);

for (const packageName of fs.readdirSync(runtimeModules)) {
  const packageLoc = path.join(runtimeModules, packageName);
  const files = [];
  for (const filename of fs.readdirSync(packageLoc)) {
    files.push([
      filename,
      fs.readFileSync(path.join(packageLoc, filename), 'utf8'),
    ]);
  }

  runtimeIndexFile += `modules.set('${packageName}', new Map(${JSON.stringify(
    files,
    null,
    '  ',
  )}));\n`;
}

write(runtimeIndex, runtimeIndexFile);
