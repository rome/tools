/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

require('../_setup.cjs');

const path = require('path');
const fs = require('fs');

const {
	formatterFolder,
	analysisFolder,
	astFolder,
} = require('../_constants.cjs');

const {write, getBuilderName} = require('../_utils.cjs');

const nodeType = process.argv[2];
const category = process.argv[3];
if (nodeType === undefined || category === undefined) {
	console.error('node scripts/ast/add.cjs [node-type] [category]');
	process.exit(1);
}

const builderName = getBuilderName(nodeType);

// Write AST def
let file = `/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type ${nodeType} = JSNodeBase & {
  type: '${nodeType}';
};

export const ${builderName} = createBuilder<${nodeType}>('${nodeType}', {
  bindingKeys: {},
  visitorKeys: {},
});
`;
const fileLoc = path.join(astFolder, category, `${nodeType}.ts`);
if (fs.existsSync(fileLoc, 'utf8')) {
	console.log('Already have', nodeType);
	process.exit();
}
write(fileLoc, file);

// Write builder
const builderDefFile = path.join(formatterFolder, category, `${nodeType}.ts`);
const builderContent = `/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import Builder from '../../Builder';
import {AnyNode, ${nodeType}} from '@romejs/js-ast';
import {Token} from '../../tokens';

export default function ${nodeType}(builder: Builder, node: ${nodeType}): Token {
  throw new Error('unimplemented');
}
`;
write(builderDefFile, builderContent);

// Write analysis
const analysisDefFile = path.join(analysisFolder, category, `${nodeType}.ts`);
const analysisContent = `/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {AnyNode, ${nodeType}, ${builderName}} from '@romejs/js-ast';

export default function ${nodeType}(node: AnyNode) {
  node = ${builderName}.assert(node);
  throw new Error('unimplemented');
}
 `;
write(analysisDefFile, analysisContent);

require('./update.cjs');
