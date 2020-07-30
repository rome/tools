/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {ConstBinding, LetBinding} from "@internal/compiler";
import {AnyNode, jsVariableDeclaration} from "@internal/ast";
import {getBindingIdentifiers} from "@internal/js-ast-utils";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	inject(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = jsVariableDeclaration.assert(node);

		for (const decl of node.declarations) {
			for (const id of getBindingIdentifiers(decl)) {
				switch (node.kind) {
					case "let": {
						scope.addBinding(
							new LetBinding({
								node: id,
								name: id.name,
								scope,
							}),
						);
						break;
					}

					case "const": {
						// Only set the value for simple declarations
						let valueNode = id === decl.id ? decl.init : undefined;
						scope.addBinding(
							new ConstBinding(
								{
									node: id,
									name: id.name,
									scope,
								},
								valueNode,
							),
						);
						break;
					}

					case "var": {
						// Should be injected manually by `addVarBindings`
						break;
					}
				}
			}
		}
	},
});
