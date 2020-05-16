/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';
import {createUnknownFilePath} from '@romejs/path';
import {filenameToId} from './defaultExportSameBasename';

export default {
	name: 'importDefaultBasename',
	enter(path: Path): AnyNode {
		const {node} = path;

		if (node.type === 'ImportDeclaration') {
			const {defaultSpecifier} = node;
			if (defaultSpecifier === undefined) {
				return node;
			}

			const expectedName = filenameToId(
				createUnknownFilePath(node.source.value),
				false,
			);
			if (expectedName === undefined) {
				return node;
			}

			const localName = defaultSpecifier.local.name.name;
			if (localName !== expectedName) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.IMPORT_DEFAULT_BASENAME(localName, expectedName),
				);
			}
		}

		return node;
	},
};
