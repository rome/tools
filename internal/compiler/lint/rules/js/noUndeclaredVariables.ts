/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

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

// This is gross...
const TS_VARIABLES = [
	"MethodDecorator",
	"ParameterDecorator",
	"PromiseConstructorLike",
	"PromiseLike",
	"Promise",
	"ArrayLike",
	"Partial",
	"Required",
	"Readonly",
	"Pick",
	"Record",
	"Exclude",
	"Extract",
	"Omit",
	"NonNullable",
	"Parameters",
	"ConstructorParameters",
	"ReturnType",
	"InstanceType",
	"ThisType",
	"NodeJS",
	"Iterable",
	"IterableIterator",
	"ArrayBufferView",
	"Iterator",
	"TemplateStringsArray",
	"BufferEncoding",
	"Console",
	"Thenable",
];

export default createVisitor({
	name: "js/noUndeclaredVariables",
	enter(path) {
		const {node, scope} = path;

		if (
			node.type === "JSReferenceIdentifier" ||
			node.type === "JSXReferenceIdentifier"
		) {
			const {name} = node;
			const binding = scope.getBinding(name);

			const isDefined =
				binding !== undefined ||
				scope.isGlobal(name) ||
				BROWSER_VARIABLES.includes(name) ||
				NODE_VARIABLES.includes(name) ||
				TS_VARIABLES.includes(name);

			if (!isDefined) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.JS_NO_UNDECLARED_VARIABLES(
						name,
						scope.getBindingNames(),
					),
					{
						meta: {
							identifierName: name,
						},
					},
				);
			}
		}

		return signals.retain;
	},
});
