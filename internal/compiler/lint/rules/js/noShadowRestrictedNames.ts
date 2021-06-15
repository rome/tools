/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createLintVisitor, signals} from "@internal/compiler";
import {
	builtin,
	es5,
	es2015,
	es2017,
	es2020,
	es2021,
} from "@internal/compiler/scope/globals";
import {descriptions} from "@internal/diagnostics";

const restrictedNames = new Set([
	...builtin,
	...es5,
	...es2015,
	...es2017,
	...es2020,
	...es2021,
]);

export default createLintVisitor({
	name: "js/noShadowRestrictedNames",
	enter(path) {
		const {node, context, scope} = path;

		if (scope.node === node) {
			for (const [name, binding] of scope.getOwnBindings()) {
				if (restrictedNames.has(name)) {
					context.addNodeDiagnostic(
						binding.node,
						descriptions.LINT.JS_NO_SHADOW_RESTRICTED_NAMES(name),
					);
				}
			}
		}

		return signals.retain;
	},
});
