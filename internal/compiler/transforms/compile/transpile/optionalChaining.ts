/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {template} from "@internal/js-ast-utils";
import {
	AnyJSExpression,
	JSComputedMemberProperty,
	JSOptionalCallExpression,
	JSStaticMemberProperty,
	jsCallExpression,
	jsMemberExpression,
} from "@internal/ast";

function unoptionifyMemberExpression(node: AnyJSExpression): AnyJSExpression {
	let root: null | AnyJSExpression = null;
	const properties: (JSStaticMemberProperty | JSComputedMemberProperty)[] = [];
	let current: AnyJSExpression = node;

	while (true) {
		if (current.type === "JSMemberExpression") {
			properties.unshift({
				...current.property,
				optional: false,
			});
			current = current.object;
		} else {
			root = current;
			break;
		}
	}

	if (root === null) {
		throw new Error(
			"optionalChaining transform: js member expression root cannot be null",
		);
	}

	return properties.reduce(
		(object, property) =>
			jsMemberExpression.create({
				object,
				property,
			})
		,
		(root as AnyJSExpression),
	);
}

function findParts(
	node: JSOptionalCallExpression,
): {
	needsChecking: AnyJSExpression;
	callee: AnyJSExpression;
} {
	let needsChecking: AnyJSExpression | null = null;
	const properties: (JSStaticMemberProperty | JSComputedMemberProperty)[] = [];

	let current = node.callee;
	while (true) {
		if (current.type === "JSMemberExpression") {
			if (current.property.optional === true) {
				needsChecking = current.object;
				properties.unshift({
					...current.property,
					optional: false,
				});
				break;
			} else {
				properties.unshift(current.property);
				current = current.object;
			}
		}

		throw new Error("Not implemented non js memeber expression case");
	}

	const callee = properties.reduce(
		(accum, property) => {
			return jsMemberExpression.create({
				object: accum,
				property,
			});
		},
		unoptionifyMemberExpression(needsChecking),
	);

	return {
		needsChecking,
		callee,
	};
}

export default createVisitor({
	name: "optionalChaining",
	enter(path) {
		const {node} = path;

		if (node.type === "JSMemberExpression" && node.property.optional) {
			// TODO assign `node.object` to a variable and use it as a reference
			if (node.property.type === "JSComputedMemberProperty") {
				return signals.replace(
					template.expression`${node.object} == null ? undefined : ${node.object}[${node.property.value}]`,
				);
			} else {
				return signals.replace(
					template.expression`${node.object} == null ? undefined : ${node.object}.${node.property.value}`,
				);
			}
		}

		if (node.type === "JSOptionalCallExpression") {
			// TODO assign `node.callee` to a variable and use it as a reference
			if (node.optional) {
				return signals.replace(
					template.expression`${node.callee} == null ? undefined : ${jsCallExpression.create({
						callee: unoptionifyMemberExpression(node.callee),
						arguments: node.arguments,
					})}`,
				);
			} else {
				const {needsChecking, callee} = findParts(node);
				return signals.replace(
					template.expression`${needsChecking} == null ? undefined : ${jsCallExpression.create({
						callee,
						arguments: node.arguments,
					})}`,
				);
			}
		}

		return signals.retain;
	},
});
