/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {toCamelCase} from "@internal/string-utils";
import {Binding} from "@internal/compiler/scope/bindings";
import {descriptions} from "@internal/diagnostics";
import {isValidIdentifierName, renameBindings} from "@internal/js-ast-utils";

export function normalizeCamelCase(name: string): undefined | string {
	if (!isValidIdentifierName(name)) {
		return undefined;
	}

	if (name === "") {
		return undefined;
	}

	return name;
}

export default createVisitor({
	name: "js/useCamelCase",
	enter(path) {
		const {node, scope, context} = path;

		// Check variables
		if (node === scope.node) {
			const renames: Map<Binding, string> = new Map();

			for (const [name, binding] of scope.getOwnBindings()) {
				const camelName = normalizeCamelCase(
					toCamelCase(
						name,
						{
							allowShouty: true,
						},
					),
				);
				if (camelName !== undefined && camelName !== name) {
					const {suppressed} = context.addNodeDiagnostic(
						binding.node,
						descriptions.LINT.JS_VARIABLE_CAMEL_CASE(name, camelName),
						{tags: {fixable: true}},
					);
					if (!suppressed) {
						renames.set(binding, camelName);
					}
				}
			}

			if (renames.size > 0) {
				return signals.replace(renameBindings(path, renames));
			}
		}

		return signals.retain;
	},
});
