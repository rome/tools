/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from '../Scope';
import {AnyNode, SwitchCase} from '@romejs/js-ast';

export default {
	creator: false,
	build(node: SwitchCase, parent: AnyNode, scope: Scope) {
		for (const child of node.consequent) {
			scope.evaluate(child, node);
		}
	},
};
