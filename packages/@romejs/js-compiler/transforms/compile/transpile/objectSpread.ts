/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from "@romejs/js-compiler";
import {
	AnyNode,
	AnyObjectMember,
	AnyStatement,
	AnyTargetBindingPattern,
	BindingIdentifier,
	CallExpression,
	ObjectExpression,
	VariableDeclaration,
	VariableDeclarationStatement,
	VariableDeclarator,
	bindingIdentifier,
	bindingObjectPattern,
	callExpression,
	objectExpression,
	referenceIdentifier,
	variableDeclaration,
	variableDeclarationStatement,
	variableDeclarator,
} from "@romejs/js-ast";
import {template} from "@romejs/js-ast-utils";

function hasSpreadProperty(props: Array<AnyNode>): boolean {
	for (const prop of props) {
		if (prop.type === "SpreadProperty") {
			return true;
		}
	}
	return false;
}

function getRestProperty(
	node:
		| undefined
		| VariableDeclarator
		| VariableDeclarationStatement
		| VariableDeclaration
		| AnyTargetBindingPattern,
): undefined | BindingIdentifier {
	if (node === undefined) {
		return undefined;
	}

	switch (node.type) {
		case "VariableDeclarator":
			return getRestProperty(node.id);

		case "VariableDeclarationStatement":
			return getRestProperty(node.declaration);

		case "VariableDeclaration": {
			for (const declarator of node.declarations) {
				const rest = getRestProperty(declarator);
				if (rest !== undefined) {
					return rest;
				}
			}
			return undefined;
		}

		case "BindingObjectPattern":
			return node.rest;
	}

	return undefined;
}

function transformSpreadProperty(
	path: Path,
	node: ObjectExpression,
): CallExpression {
	let props: Array<AnyObjectMember> = [];
	const assignArgs = [];

	function pushProps() {
		if (props.length === 0 && assignArgs.length > 0) {
			return;
		}

		assignArgs.push(objectExpression.create({properties: props}));

		props = [];
	}

	for (const prop of node.properties) {
		if (prop.type === "SpreadProperty") {
			pushProps();
			assignArgs.push(prop.argument);
		} else {
			props.push(prop);
		}
	}

	pushProps();

	return callExpression.create({
		callee: template.expression`Object.assign`,
		arguments: assignArgs,
	});
}

function transformRestProperty(
	path: Path,
	node: VariableDeclaration,
): Array<AnyStatement> {
	const nodes: Array<AnyStatement> = [];

	for (const declarator of node.declarations) {
		const restElem = getRestProperty(declarator);

		if (restElem === undefined || declarator.id.type !== "BindingObjectPattern") {
			nodes.push(
				variableDeclarationStatement.quick(
					variableDeclaration.create({
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
			variableDeclarationStatement.quick(
				variableDeclaration.create({
					kind: node.kind,
					declarations: [
						variableDeclarator.create({
							id: bindingIdentifier.create({
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
			if (prop.type === "BindingObjectPatternProperty") {
				if (
					prop.key.type === "ComputedPropertyKey" ||
					prop.key.value.type !== "Identifier"
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
			variableDeclarationStatement.quick(
				variableDeclaration.create({
					kind: node.kind,
					declarations: [
						variableDeclarator.create({
							id: bindingIdentifier.quick(restName),
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
			variableDeclarationStatement.quick(
				variableDeclaration.create({
					kind: node.kind,
					declarations: [
						variableDeclarator.create({
							id: bindingObjectPattern.create({
								properties: declarator.id.properties,
								rest: undefined,
							}),
							init: referenceIdentifier.quick(uid),
						}),
					],
				}),
			),
		);
	}

	return nodes;
}

export default {
	name: "objectSpread",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (
			node.type === "VariableDeclarationStatement" &&
			getRestProperty(node) !== undefined
		) {
			return transformRestProperty(path, node.declaration);
		}

		if (node.type === "ObjectExpression" && hasSpreadProperty(node.properties)) {
			return transformSpreadProperty(path, node);
		}

		return node;
	},
};
