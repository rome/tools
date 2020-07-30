/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ImportBinding, Path, createVisitor, signals} from "@internal/compiler";
import {
	JSExportExternalDeclaration,
	JSExportLocalDeclaration,
	jsExportExternalDeclaration,
	jsExportExternalSpecifier,
	jsExportLocalDeclaration,
	jsIdentifier,
	jsStringLiteral,
} from "@internal/ast";

export default createVisitor({
	name: "optimizeExports",
	enter(path: Path) {
		const {node} = path;

		// turn `import {a} from 'b'; export {a}`; to `export {a} from 'b';`';
		if (
			node.type === "JSExportLocalDeclaration" &&
			node.exportKind === "value" &&
			node.declaration === undefined &&
			node.specifiers !== undefined
		) {
			const nodes: Array<JSExportExternalDeclaration | JSExportLocalDeclaration> = [];
			const specifiers = [];

			for (const specifier of node.specifiers) {
				if (specifier.type === "JSExportLocalSpecifier") {
					const binding = path.scope.getBinding(specifier.local.name);
					if (
						binding !== undefined &&
						binding instanceof ImportBinding &&
						binding.meta.type === "name"
					) {
						nodes.push(
							jsExportExternalDeclaration.create({
								namedSpecifiers: [
									jsExportExternalSpecifier.create({
										local: jsIdentifier.quick(binding.meta.imported),
										exported: specifier.exported,
										loc: specifier.loc,
									}),
								],
								source: jsStringLiteral.quick(binding.meta.source),
							}),
						);
					} else {
						specifiers.push(specifier);
					}
				} else {
					// TODO ???
					specifiers.push(specifier);
				}
			}

			if (specifiers.length === node.specifiers.length && nodes.length === 0) {
				return signals.retain;
			}

			if (specifiers.length !== 0) {
				nodes.push(jsExportLocalDeclaration.create({specifiers}));
			}

			return signals.replace(nodes);
		}

		return signals.retain;
	},
});
