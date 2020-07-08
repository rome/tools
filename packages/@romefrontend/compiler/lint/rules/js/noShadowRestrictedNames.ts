/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romefrontend/compiler";
import {TransformExitResult} from "@romefrontend/compiler/types";
import {
	builtin,
	es5,
	es2015,
	es2017,
} from "@romefrontend/compiler/scope/globals";
import {descriptions} from "@romefrontend/diagnostics";

const restrictedNames = new Set([...builtin, ...es5, ...es2015, ...es2017]);

export default {
	name: "noShadowRestrictedNames",
	enter(path: Path): TransformExitResult {
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

		return node;
	},
};
