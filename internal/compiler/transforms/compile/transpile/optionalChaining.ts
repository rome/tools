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
	JSMemberExpression,
	JSOptionalCallExpression,
	JSStaticMemberProperty,
	jsCallExpression,
	jsMemberExpression,
} from "@internal/ast";

function unoptionifyMemberExpression(node: AnyJSExpression): AnyJSExpression {
	let root!: AnyJSExpression;
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
	node: JSOptionalCallExpression | JSMemberExpression,
): {
	needsChecking: AnyJSExpression | null;
	unoptionifedMembers: AnyJSExpression | null;
} {
	let needsChecking: AnyJSExpression | null = null;
	const properties: (JSStaticMemberProperty | JSComputedMemberProperty)[] = [];

	let current = node.type === "JSOptionalCallExpression" ? node.callee : node;

	while (current.type === "JSMemberExpression") {
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

	const unoptionifedMembers =
		needsChecking &&
		properties.reduce(
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
		unoptionifedMembers,
	};
}

export default createVisitor({
	name: "optionalChaining",
	enter(path) {
		const {node} = path;

		if (node.type === "JSMemberExpression") {
			const {needsChecking, unoptionifedMembers} = findParts(node);
			if (needsChecking && unoptionifedMembers) {
				return signals.replace(
					template.expression`${needsChecking} == void 0 ? void 0 : ${unoptionifedMembers}`,
				);
			}
		}
		if (node.type === "JSOptionalCallExpression") {
			if (node.optional) {
				return signals.replace(
					template.expression`${node.callee} == void 0 ? void 0 : ${jsCallExpression.create({
						callee: unoptionifyMemberExpression(node.callee),
						arguments: node.arguments,
					})}`,
				);
			} else {
				const {needsChecking, unoptionifedMembers} = findParts(node);
				if (needsChecking && unoptionifedMembers) {
					return signals.replace(
						template.expression`${needsChecking} == void 0 ? void 0 : ${jsCallExpression.create({
							callee: unoptionifedMembers,
							arguments: node.arguments,
						})}`,
					);
				}
			}
		}
		return signals.retain;
	},
});
