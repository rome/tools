/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {
	AnyNode,
	JSXAttribute,
	JSXSpreadAttribute,
	ObjectMethod,
	ObjectProperty,
	SpreadProperty,
} from '@romejs/js-ast';
import {doesNodeMatchPattern} from '@romejs/js-ast-utils';
import {descriptions} from '@romejs/diagnostics';
function isAttributePassingChildrenProp(
	attribute: JSXAttribute | JSXSpreadAttribute,
): boolean {
	return attribute.type === 'JSXAttribute' && attribute.name.name === 'children';
}
function isCreateElementPassingChildrenProp(
	property: ObjectProperty | ObjectMethod | SpreadProperty,
): boolean {
	return (
		property.type === 'ObjectProperty' &&
		property.key.value.type === 'Identifier' &&
		property.key.value.name === 'children'
	);
}
export default {
	name: 'noChildrenProp',
	enter(path: Path): AnyNode {
		const {node} = path;
		if (
			(node.type === 'JSXElement' &&
			node.attributes.find(isAttributePassingChildrenProp)) ||
			(node.type === 'CallExpression' &&
			doesNodeMatchPattern(node.callee, 'React.createElement') &&
			node.arguments[1].type === 'ObjectExpression' &&
			node.arguments[1].properties.find(isCreateElementPassingChildrenProp))
		) {
			path.context.addNodeDiagnostic(node, descriptions.LINT.NO_CHILDREN_PROP);
		}

		return node;
	},
};
