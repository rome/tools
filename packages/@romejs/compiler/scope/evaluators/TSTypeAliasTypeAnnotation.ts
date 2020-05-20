/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {TypeBinding} from "@romejs/compiler";
import {AnyNode, TSTypeAliasTypeAnnotation} from "@romejs/ast";

export default {
	creator: false,
	build(node: TSTypeAliasTypeAnnotation, parent: AnyNode, scope: Scope) {
		scope.evaluate(node.typeParameters);
		scope.addBinding(
			new TypeBinding(
				{
					node: node.id,
					name: node.id.name,
					scope,
				},
				node,
				"typealias",
			),
		);
	},
};
