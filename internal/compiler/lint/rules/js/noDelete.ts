/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {
	jsAssignmentExpression,
	jsMemberExpression,
	jsReferenceIdentifier,
} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";

export default createVisitor({
	name: "js/noDelete",
	enter(path) {
		const {node} = path;

		if (
			node.type === "JSUnaryExpression" &&
			node.operator === "delete" &&
			node.argument.type === "JSMemberExpression"
		) {
			const left = node.argument;
			return path.addFixableDiagnostic(
				{
					fixed: signals.replace(
						jsAssignmentExpression.create(
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
					),
				},
				descriptions.LINT.JS_NO_DELETE,
			);
		}

		return signals.retain;
	},
});
