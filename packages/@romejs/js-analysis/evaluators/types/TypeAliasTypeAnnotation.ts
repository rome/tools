/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
	AnyNode,
	TypeAliasTypeAnnotation,
	typeAliasTypeAnnotation,
} from '@romejs/js-ast';

export default function TypeAliasTypeAnnotation(node: AnyNode, scope: Scope) {
	node = typeAliasTypeAnnotation.assert(node);

	const typeScope = scope.fork();
	if (node.typeParameters) {
		typeScope.evaluate(node.typeParameters);
	}

	const right = typeScope.evaluate(node.right);
	scope.addBinding(node.id.name, right);
	return right;
}
