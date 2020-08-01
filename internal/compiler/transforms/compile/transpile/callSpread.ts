/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {template} from "@internal/js-ast-utils";
import {
	JSCallExpression,
	JSNullLiteral,
	JSReferenceIdentifier,
	jsArrayExpression,
	jsAssignmentExpression,
	jsMemberExpression,
	jsNullLiteral,
	jsSequenceExpression,
} from "@internal/ast";
import {injectBinding} from "../../helpers";

export default createVisitor({
	name: "callSpread",
	enter(path) {
		const {node} = path;

		if (node.type === "JSCallExpression") {
			let func = node.callee;

			// Impossible to transform a bare super call
			if (func.type === "JSSuper") {
				return signals.retain;
			}

			let hasSpread = false;
			for (const arg of node.arguments) {
				if (arg.type === "JSSpreadElement") {
					hasSpread = true;
					break;
				}
			}
			if (hasSpread) {
				let prepend;

				let object: JSReferenceIdentifier | JSNullLiteral;
				if (func.type === "JSMemberExpression") {
					const injection = injectBinding(path);
					object = injection[0];

					prepend = jsAssignmentExpression.create({
						operator: "=",
						left: injection[1],
						right: func.object,
					});

					func = jsMemberExpression.create({
						object,
						property: func.property,
					});
				} else {
					object = jsNullLiteral.create({});
				}

				let call: JSCallExpression = {
					type: "JSCallExpression",
					loc: node.loc,
					callee: template.expression`${func}.apply`,
					arguments: [
						object,
						jsArrayExpression.create({elements: node.arguments}),
					],
				};

				if (prepend === undefined) {
					return signals.replace(call);
				} else {
					return signals.replace(
						jsSequenceExpression.create({
							expressions: [prepend, call],
						}),
					);
				}
			}
		}

		return signals.retain;
	},
});
