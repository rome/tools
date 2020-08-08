/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {isInTypeAnnotation} from "@internal/js-ast-utils";
import {descriptions} from "@internal/diagnostics";

const RESTRICTED_GLOBALS = ["event", "error"];

export default createVisitor({
	name: "js/restrictedGlobal",
	enter(path) {
		const {node, scope} = path;

		if (
			(node.type === "JSReferenceIdentifier" ||
			node.type === "JSXReferenceIdentifier") &&
			!isInTypeAnnotation(path)
		) {
			const {name} = node;
			const binding = scope.getBinding(name);

			const isDefined = binding !== undefined;
			const isAGlobal = scope.getRootScope().isGlobal(name);

			if (!isDefined && isAGlobal && RESTRICTED_GLOBALS.includes(name)) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.JS_NO_RESTRICTED_GLOBALS(name),
				);
			}
		}

		return signals.retain;
	},
});
