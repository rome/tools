/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createLintVisitor, signals} from "@internal/compiler";
import {
	JSVariableDeclarationStatement,
	jsVariableDeclaration,
	jsVariableDeclarationStatement,
} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";

export default createLintVisitor({
	name: "js/useSingleVarDeclarator",
	enter(path) {
		const {node} = path;

		if (
			node.type === "JSVariableDeclarationStatement" &&
			node.declaration.declarations.length > 1
		) {
			const fixed: JSVariableDeclarationStatement[] = [];
			const {kind} = node.declaration;

			for (const declarator of node.declaration.declarations) {
				fixed.push(
					jsVariableDeclarationStatement.quick(
						jsVariableDeclaration.create({
							kind,
							declarations: [declarator],
						}),
					),
				);
			}

			return path.addFixableDiagnostic(
				{fixed: signals.replace(fixed)},
				descriptions.LINT.JS_USE_SINGLE_VAR_DECLARATOR,
			);
		}

		return signals.retain;
	},
});
