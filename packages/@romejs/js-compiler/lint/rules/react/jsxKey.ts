/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode, JSXElement} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

function containsKeyAttr(node: JSXElement): boolean {
	const ATTR_NAME = 'key';
	return !!node.attributes.find((attr) =>
		attr.type === 'JSXAttribute' && attr.name.name === ATTR_NAME
	);
}

export default {
	name: 'jsxKey',
	enter(path: Path): AnyNode {
		const {node, context} = path;

		// JSXElement in array literal
		if (
			node.type === 'JSXElement' &&
			!containsKeyAttr(node) &&
			path.parentPath.node.type === 'ArrayExpression'
		) {
			context.addNodeDiagnostic(node, descriptions.LINT.REACT_JSX_KEY('array'));
		}

		// Array.prototype.map
		if (
			node.type === 'CallExpression' &&
			node.callee.type === 'MemberExpression' &&
			node.callee.property.value.type === 'Identifier' &&
			node.callee.property.value.name === 'map'
		) {
			const fn = node.arguments[0];

			// Short hand arrow function
			if (
				fn.type === 'ArrowFunctionExpression' &&
				fn.body.type === 'JSXElement' &&
				!containsKeyAttr(fn.body)
			) {
				context.addNodeDiagnostic(
					fn.body,
					descriptions.LINT.REACT_JSX_KEY('iterator'),
				);
			}

			// Function or arrow function with block statement
			if (
				fn &&
				(fn.type === 'FunctionExpression' || fn.type === 'ArrowFunctionExpression') &&
				fn.body.type === 'BlockStatement'
			) {
				fn.body.body.forEach((statement) => {
					if (
						statement.type === 'ReturnStatement' &&
						statement.argument?.type === 'JSXElement' &&
						!containsKeyAttr(statement.argument)
					) {
						context.addNodeDiagnostic(
							statement.argument,
							descriptions.LINT.REACT_JSX_KEY('iterator'),
						);
					}
				});
			}
		}

		return node;
	},
};
