/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {ConstBinding, LetBinding, VarBinding} from "@romejs/compiler";
import {AnyNode, jsVariableDeclaration} from "@romejs/ast";
import {getBindingIdentifiers} from "@romejs/js-ast-utils";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	inject(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = jsVariableDeclaration.assert(node);

		for (const decl of node.declarations) {
			for (const id of getBindingIdentifiers(decl)) {
				if (node.kind === "let") {
					scope.addBinding(
						new LetBinding({
							node: id,
							name: id.name,
							scope,
						}),
					);
				}

				if (node.kind === "const") {
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
				}

				if (
					node.kind === "var" &&
					(scope.kind === "program" || scope.kind === "block")
				) {
					if (!scope.hasHoistedVars) {
						throw new Error(
							"This scope does not allow `var`iables. This is probably because `var`iables were injected into a scope that did not contain `var` in the original source." +
							scope.kind,
						);
					}

					scope.addBinding(
						new VarBinding({
							node: id,
							name: id.name,
							scope,
						}),
					);
				}
			}
		}
	},
});
