/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from "@romejs/js-compiler";
import {
	jsAssignmentExpression,
	jsMemberExpression,
	jsReferenceIdentifier,
} from "@romejs/ast";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "noDelete",
	enter(path: Path): TransformExitResult {
		const {context, node} = path;

		if (
			node.type === "JSUnaryExpression" &&
			node.operator === "delete" &&
			node.argument.type === "JSMemberExpression"
		) {
			const left = node.argument;
			return context.addFixableDiagnostic(
				{
					old: node,
					fixed: jsAssignmentExpression.create(
						{
							operator: "=",
							left: jsMemberExpression.create({
								object: left.object,
								property: left.property,
							}),
							right: jsReferenceIdentifier.create({
								name: "undefined",
							}),
						},
						node,
					),
				},
				descriptions.LINT.JS_NO_DELETE,
			);
		}

		return node;
	},
};
