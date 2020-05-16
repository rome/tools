/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from '../Scope';
import {AnyNode, BlockStatement} from '@romejs/js-ast';
import {isFunctionNode} from '@romejs/js-ast-utils';
import {addVarBindings} from '../utils';

export default {
	creator: true,
	build(node: BlockStatement, parent: AnyNode, scope: Scope) {
		if (isFunctionNode(parent) && scope.node !== parent) {
			scope = scope.evaluate(parent.head, parent, true);
		}

		const newScope = scope.fork('block', node);

		if (isFunctionNode(parent) && parent.head.hasHoistedVars) {
			addVarBindings(newScope, parent);
		}

		for (const child of node.body) {
			newScope.evaluate(child, node);
		}

		return newScope;
	},
};
