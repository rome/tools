/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from "@romejs/js-compiler";
import {
	VariableDeclarationStatement,
	variableDeclaration,
	variableDeclarationStatement,
} from "@romejs/js-ast";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "singleVarDeclarator",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (
			node.type === "VariableDeclarationStatement" &&
			node.declaration.declarations.length > 1
		) {
			const fixed: Array<VariableDeclarationStatement> = [];
			const {kind} = node.declaration;

			for (const declarator of node.declaration.declarations) {
				fixed.push(
					variableDeclarationStatement.quick(
						variableDeclaration.create({
							kind,
							declarations: [declarator],
						}),
					),
				);
			}

			return path.context.addFixableDiagnostic(
				{old: node, fixed},
				descriptions.LINT.JAVASCRIPT_SINGLE_VAR_DECLARATOR,
			);
		}

		return node;
	},
};
