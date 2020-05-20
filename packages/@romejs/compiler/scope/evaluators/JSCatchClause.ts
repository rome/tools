/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {LetBinding} from "@romejs/compiler";
import {getBindingIdentifiers} from "@romejs/js-ast-utils";
import {AnyNode, JSCatchClause} from "@romejs/ast";

export default {
	creator: true,
	build(node: JSCatchClause, parent: AnyNode, scope: Scope) {
		const newScope = scope.fork("block", node);
		if (node.param !== undefined) {
			for (const id of getBindingIdentifiers(node.param)) {
				newScope.addBinding(
					new LetBinding(
						{
							node: id,
							name: id.name,
							scope: newScope,
						},
						"catch",
					),
				);
			}
		}
		return newScope;
	},
};
