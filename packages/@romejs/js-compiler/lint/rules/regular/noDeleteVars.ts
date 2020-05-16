/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

export default {
	name: 'noDeleteVars',
	enter(path: Path): AnyNode {
		const {node} = path;

		if (
			node.type === 'UnaryExpression' &&
			node.operator === 'delete' &&
			node.argument.type === 'ReferenceIdentifier'
		) {
			path.context.addNodeDiagnostic(node, descriptions.LINT.NO_DELETE_VARS);
		}

		return node;
	},
};
