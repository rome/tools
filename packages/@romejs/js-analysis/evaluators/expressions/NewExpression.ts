/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {AnyNode, NewExpression, newExpression} from '@romejs/js-ast';
import InstanceT from '../../types/InstanceT';

export default function NewExpression(node: AnyNode, scope: Scope) {
	node = newExpression.assert(node);
	return new InstanceT(scope, node, scope.evaluate(node.callee), []);
}
