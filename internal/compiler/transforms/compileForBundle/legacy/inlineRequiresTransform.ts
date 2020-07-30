/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSStatement, JSVariableDeclarator} from "@internal/ast";
import {ConstBinding, createVisitor, signals} from "@internal/compiler";
import {getRequireSource, isInTypeAnnotation} from "@internal/js-ast-utils";

const NON_INLINED_REQUIRES: Array<string> = [];

export default createVisitor({
	name: "inlineRequiresTransform",
	enter(path) {
		const {node} = path;

		if (node.type === "JSReferenceIdentifier") {
			const binding = path.scope.getBinding(node.name);

			// Inline references to a require variable
			if (binding !== undefined && binding instanceof ConstBinding) {
				const source = getRequireSource(binding.value, path.scope, true);
				if (
					source !== undefined &&
					!NON_INLINED_REQUIRES.includes(source) &&
					!isInTypeAnnotation(path) &&
					binding.value !== undefined
				) {
					return signals.replace(binding.value);
				}
			}
		}

		return signals.retain;
	},
	exit(path) {
		const {node} = path;

		if (node.type === "JSRoot" || node.type === "JSBlockStatement") {
			const body: Array<AnyJSStatement> = [];
			let hadRequires = false;

			// Remove all require declarations that could have been inlined
			for (const bodyNode of node.body) {
				if (
					bodyNode.type === "JSVariableDeclarationStatement" &&
					bodyNode.declaration.kind === "const"
				) {
					let hadRequireDeclarators = false;
					const declarators: Array<JSVariableDeclarator> = [];

					for (const decl of bodyNode.declaration.declarations) {
						if (decl.id.type !== "JSBindingIdentifier") {
							// Patterns aren't supported yet
							declarators.push(decl);
							continue;
						}

						const source = getRequireSource(decl.init, path.scope, true);
						if (source === undefined) {
							// Didn't contain a `require`
							declarators.push(decl);
							continue;
						}

						if (NON_INLINED_REQUIRES.includes(source)) {
							declarators.push(decl);
							continue;
						}

						hadRequireDeclarators = true;
						hadRequires = true;
					}

					if (hadRequireDeclarators) {
						if (declarators.length > 0) {
							body.push({
								...bodyNode,
								declaration: {
									...bodyNode.declaration,
									declarations: declarators,
								},
							});
						}
						continue;
					}
				}

				body.push(bodyNode);
			}

			if (!hadRequires) {
				return signals.retain;
			}

			return signals.replace({
				...node,
				body,
			});
		}

		return signals.retain;
	},
});
