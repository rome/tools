/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from "@romejs/compiler";
import {toCamelCase} from "@romejs/string-utils";
import {Binding} from "@romejs/compiler/scope/bindings";
import {descriptions} from "@romejs/diagnostics";
import {
	isIdentifierish,
	isValidIdentifierName,
	isVariableIdentifier,
	renameBindings,
} from "@romejs/js-ast-utils";

function normalizeCamelCase(name: string): undefined | string {
	if (!isValidIdentifierName(name)) {
		return undefined;
	}

	if (name === "") {
		return undefined;
	}

	return name;
}

// Allow prefixed underscores
export function toVariableCamelCase(
	name: string,
	forceCapitalize?: boolean,
): undefined | string {
	// Allow shouty constants
	if (name.toUpperCase() === name) {
		return normalizeCamelCase(name);
	}

	let prefix = "";
	let suffix = "";

	const prefixDashes = name.match(/^_+/);
	if (prefixDashes != null) {
		prefix = prefixDashes[0];
	}

	const suffixDashes = name.match(/_+$/);
	if (suffixDashes != null) {
		suffix = suffixDashes[0];
	}

	// Remove prefix and suffix
	let slicedName = name.slice(prefix.length);
	if (suffix.length > 0) {
		slicedName = slicedName.slice(0, -suffix.length);
	}

	const camelName = prefix + toCamelCase(slicedName, forceCapitalize) + suffix;
	return normalizeCamelCase(camelName);
}

export default {
	name: "camelCase",
	enter(path: Path): TransformExitResult {
		const {node, scope, context} = path;

		// Check variables
		if (node === scope.node) {
			const renames: Map<Binding, string> = new Map();

			for (const [name, binding] of scope.getOwnBindings()) {
				const camelName = toVariableCamelCase(name);
				if (camelName !== undefined && camelName !== name) {
					const {suppressed} = context.addNodeDiagnostic(
						binding.node,
						descriptions.LINT.JS_VARIABLE_CAMEL_CASE(name, camelName),
						{fixable: true},
					);
					if (!suppressed) {
						renames.set(binding, camelName);
					}
				}
			}

			if (renames.size > 0) {
				return renameBindings(path, renames);
			}
		}

		// Check regular jsIdentifiers, variable jsIdentifiers have already been checked above
		if (isIdentifierish(node) && !isVariableIdentifier(node)) {
			const {name} = node;
			const camelName = toVariableCamelCase(name);
			if (camelName !== undefined && camelName !== name) {
				return context.addFixableDiagnostic(
					{
						old: node,
						suggestions: [
							{
								title: "Convert to camelCase",
								description: "This may not be safe. Are you passing this into a third party module?",
								fixed: {...node, name: camelName},
							},
						],
					},
					descriptions.LINT.JS_IDENTIFIER_CAMEL_CASE(name, camelName),
				);
			}
		}

		return node;
	},
};
