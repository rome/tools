/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romefrontend/compiler";
import {AnyNode} from "@romefrontend/ast";
import {isInTypeAnnotation} from "@romefrontend/js-ast-utils";
import {descriptions} from "@romefrontend/diagnostics";

const RESTRICTED_GLOBALS = ["event", "error"];

export default {
	name: "restrictedGlobal",
	enter(path: Path): AnyNode {
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
					descriptions.LINT.JS_RESTRICTED_GLOBALS(name),
				);
			}
		}

		return node;
	},
};
