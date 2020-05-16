/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from '@romejs/js-compiler';
import {unaryExpression} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

export default {
	name: 'unsafeNegation',
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (
			node.type === 'BinaryExpression' &&
			(node.operator === 'in' || node.operator === 'instanceof') &&
			node.left.type === 'UnaryExpression' &&
			node.left.operator === '!'
		) {
			return path.context.addFixableDiagnostic(
				{
					old: node,
					fixed: unaryExpression.create({
						operator: node.left.operator,
						argument: {
							...node,
							left: node.left.argument,
						},
					}),
				},
				descriptions.LINT.UNSAFE_NEGATION,
			);
		}

		return node;
	},
};
