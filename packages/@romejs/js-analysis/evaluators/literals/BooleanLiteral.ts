/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {AnyNode, BooleanLiteral, booleanLiteral} from '@romejs/js-ast';
import BooleanLiteralT from '../../types/BooleanLiteralT';

export default function BooleanLiteral(node: AnyNode, scope: Scope) {
	node = booleanLiteral.assert(node);
	return new BooleanLiteralT(scope, node, node.value);
}
