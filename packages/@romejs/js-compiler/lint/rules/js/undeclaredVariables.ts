/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/js-compiler";
import {isInTypeAnnotation} from "@romejs/js-ast-utils";
import {AnyNode} from "@romejs/js-ast";
import {descriptions} from "@romejs/diagnostics";

const NODE_VARIABLES = [
	"require",
	"__dirname",
	"__filename",
	"module",
	"exports",
];

const BROWSER_VARIABLES = [
	"fetch",
	"document",
	"window",
	"Worker",
	"cancelAnimationFrame",
	"requestAnimationFrame",
	"WebSocket",
	"alert",
	"Blob",
	"navigator",
	"Element",
	"Text",
	"Document",
	"performance",
];

export default {
	name: "undeclaredVariables",
	enter(path: Path): AnyNode {
		const {node, scope} = path;

		if (
			(node.type === "ReferenceIdentifier" ||
			node.type === "JSXReferenceIdentifier") &&
			!isInTypeAnnotation(path)
		) {
			const {name} = node;
			const binding = scope.getBinding(name);

			const isDefined =
				binding !== undefined ||
				scope.getRootScope().isGlobal(name) ||
				BROWSER_VARIABLES.includes(name) ||
				NODE_VARIABLES.includes(name);

			if (!isDefined) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.JS_UNDECLARED_VARIABLES(name),
					{
						meta: {
							identifierName: name,
						},
					},
				);
			}
		}

		return node;
	},
};
