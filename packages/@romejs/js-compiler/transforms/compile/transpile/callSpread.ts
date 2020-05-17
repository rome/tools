/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/js-compiler";
import {template} from "@romejs/js-ast-utils";
import {bindingInjector} from "../../defaultHooks/index";
import {
	CallExpression,
	NullLiteral,
	ReferenceIdentifier,
	arrayExpression,
	assignmentExpression,
	memberExpression,
	nullLiteral,
	sequenceExpression,
} from "@romejs/js-ast";

export default {
	name: "callSpread",
	enter(path: Path) {
		const {node} = path;

		if (node.type === "CallExpression") {
			let func = node.callee;

			// Impossible to transform a bare super call
			if (func.type === "Super") {
				return node;
			}

			let hasSpread = false;
			for (const arg of node.arguments) {
				if (arg.type === "SpreadElement") {
					hasSpread = true;
					break;
				}
			}
			if (hasSpread) {
				let prepend;

				let object: ReferenceIdentifier | NullLiteral;
				if (func.type === "MemberExpression") {
					const injection = path.callHook(bindingInjector, {});
					object = injection[0];

					prepend = assignmentExpression.create({
						operator: "=",
						left: injection[1],
						right: func.object,
					});

					func = memberExpression.create({
						object,
						property: func.property,
					});
				} else {
					object = nullLiteral.create({});
				}

				let call: CallExpression = {
					type: "CallExpression",
					loc: node.loc,
					callee: template.expression`${func}.apply`,
					arguments: [object, arrayExpression.create({elements: node.arguments})],
				};

				if (prepend === undefined) {
					return call;
				} else {
					return sequenceExpression.create({
						expressions: [prepend, call],
					});
				}
			}
		}

		return node;
	},
};
