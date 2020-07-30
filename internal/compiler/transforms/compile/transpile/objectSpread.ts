/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, createVisitor, signals} from "@internal/compiler";
import {
	AnyJSObjectMember,
	AnyJSStatement,
	AnyJSTargetBindingPattern,
	AnyNode,
	JSBindingIdentifier,
	JSCallExpression,
	JSObjectExpression,
	JSVariableDeclaration,
	JSVariableDeclarationStatement,
	JSVariableDeclarator,
	jsBindingIdentifier,
	jsBindingObjectPattern,
	jsCallExpression,
	jsObjectExpression,
	jsReferenceIdentifier,
	jsVariableDeclaration,
	jsVariableDeclarationStatement,
	jsVariableDeclarator,
} from "@internal/ast";
import {template} from "@internal/js-ast-utils";

function hasSpreadProperty(props: Array<AnyNode>): boolean {
	for (const prop of props) {
		if (prop.type === "JSSpreadProperty") {
			return true;
		}
	}
	return false;
}

function getRestProperty(
	node:
		| undefined
		| JSVariableDeclarator
		| JSVariableDeclarationStatement
		| JSVariableDeclaration
		| AnyJSTargetBindingPattern,
): undefined | JSBindingIdentifier {
	if (node === undefined) {
		return undefined;
	}

	switch (node.type) {
		case "JSVariableDeclarator":
			return getRestProperty(node.id);

		case "JSVariableDeclarationStatement":
			return getRestProperty(node.declaration);

		case "JSVariableDeclaration": {
			for (const declarator of node.declarations) {
				const rest = getRestProperty(declarator);
				if (rest !== undefined) {
					return rest;
				}
			}
			return undefined;
		}

		case "JSBindingObjectPattern":
			return node.rest;
	}

	return undefined;
}

function transformSpreadProperty(
	path: Path,
	node: JSObjectExpression,
): JSCallExpression {
	let props: Array<AnyJSObjectMember> = [];
	const assignArgs = [];

	function pushProps() {
		if (props.length === 0 && assignArgs.length > 0) {
			return;
		}

		assignArgs.push(jsObjectExpression.create({properties: props}));

		props = [];
	}

	for (const prop of node.properties) {
		if (prop.type === "JSSpreadProperty") {
			pushProps();
			assignArgs.push(prop.argument);
		} else {
			props.push(prop);
		}
	}

	pushProps();

	return jsCallExpression.create({
		callee: template.expression`Object.assign`,
		arguments: assignArgs,
	});
}

function transformRestProperty(
	path: Path,
	node: JSVariableDeclaration,
): Array<AnyJSStatement> {
	const nodes: Array<AnyJSStatement> = [];

	for (const declarator of node.declarations) {
		const restElem = getRestProperty(declarator);

		if (
			restElem === undefined ||
			declarator.id.type !== "JSBindingObjectPattern"
		) {
			nodes.push(
				jsVariableDeclarationStatement.quick(
					jsVariableDeclaration.create({
						kind: node.kind,
						declarations: [declarator],
					}),
				),
			);
			continue;
		}

		const uid = path.scope.generateUid();

		// push on the initial declaration so we can reference it later
		nodes.push(
			jsVariableDeclarationStatement.quick(
				jsVariableDeclaration.create({
					kind: node.kind,
					declarations: [
						jsVariableDeclarator.create({
							id: jsBindingIdentifier.create({
								name: uid,
							}),
							init: declarator.init,
						}),
					],
				}),
			),
		);

		// fetch all the previous prop names
		const removeProps = [];
		for (const prop of declarator.id.properties) {
			if (prop.type === "JSBindingObjectPatternProperty") {
				if (
					prop.key.type === "JSComputedPropertyKey" ||
					prop.key.value.type !== "JSIdentifier"
				) {
					throw new Error("unimplemented");
				} else {
					removeProps.push(prop.key.value.name);
				}
			}
		}

		// clone the init to the rest element
		const restName = restElem.name;
		nodes.push(
			jsVariableDeclarationStatement.quick(
				jsVariableDeclaration.create({
					kind: node.kind,
					declarations: [
						jsVariableDeclarator.create({
							id: jsBindingIdentifier.quick(restName),
							init: template.expression`Object.assign({}, ${uid})`,
						}),
					],
				}),
			),
		);

		// `delete` the properties
		for (const name of removeProps) {
			nodes.push(template.statement`delete ${restName}.${name};`);
		}

		// push on the initial destructuring without the rest element
		nodes.push(
			jsVariableDeclarationStatement.quick(
				jsVariableDeclaration.create({
					kind: node.kind,
					declarations: [
						jsVariableDeclarator.create({
							id: jsBindingObjectPattern.create({
								properties: declarator.id.properties,
								rest: undefined,
							}),
							init: jsReferenceIdentifier.quick(uid),
						}),
					],
				}),
			),
		);
	}

	return nodes;
}

export default createVisitor({
	name: "objectSpread",
	enter(path) {
		const {node} = path;

		if (
			node.type === "JSVariableDeclarationStatement" &&
			getRestProperty(node) !== undefined
		) {
			return signals.replace(transformRestProperty(path, node.declaration));
		}

		if (node.type === "JSObjectExpression" && hasSpreadProperty(node.properties)) {
			return signals.replace(transformSpreadProperty(path, node));
		}

		return signals.retain;
	},
});
