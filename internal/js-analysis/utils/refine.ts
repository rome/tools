/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../scopes";
import {AnyNode, JSReferenceIdentifier, JSUnaryExpression} from "@internal/ast";
import T from "../types/T";
import Evaluator from "../Evaluator";
import RefinedT from "../types/RefinedT";
import UnionT from "../types/UnionT";
import RefineTypeofT from "../types/RefineTypeofT";
import {ExtendedMap} from "@internal/collections";

type TypeDefinition = {
	name: string;
	value: T;
};

type TypeDefinitions = Array<TypeDefinition>;

function typesToMap(types: TypeDefinitions): Map<string, T> {
	const map: Map<string, T> = new Map();
	for (const {name, value} of types) {
		map.set(name, value);
	}
	return map;
}

function isTypeofNode(
	node: AnyNode,
): node is JSUnaryExpression & {
	argument: JSReferenceIdentifier;
} {
	return (
		node.type === "JSUnaryExpression" &&
		node.operator === "typeof" &&
		node.argument.type === "JSReferenceIdentifier"
	);
}

function genTypes(node: AnyNode, scope: Scope): Array<TypeDefinition> {
	const evaluator: Evaluator = scope.evaluator;
	let types = [];

	switch (node.type) {
		case "JSBinaryExpression": {
			const {left, right} = node;
			switch (node.operator) {
				case "==":
					return [];

				case "!=":
					return [];

				case "===": {
					// typeof foo === 'string'
					if (isTypeofNode(left)) {
						const name = left.argument.name;
						const binding = scope.getBinding(name);
						if (binding !== undefined) {
							types.push({
								name,
								value: new RefineTypeofT(
									scope,
									node,
									evaluator.getTypeFromEvaluatedNode(right),
									binding,
								),
							});
						}
					}

					// foo === 'bar'
					if (left.type === "JSReferenceIdentifier") {
						types.push({
							name: left.name,
							value: evaluator.getTypeFromEvaluatedNode(right),
						});
					}

					// 'string' === typeof foo
					if (isTypeofNode(right)) {
						const name = right.argument.name;
						const binding = scope.getBinding(name);
						if (binding !== undefined) {
							types.push({
								name,
								value: new RefineTypeofT(
									scope,
									node,
									evaluator.getTypeFromEvaluatedNode(left),
									binding,
								),
							});
						}
					}

					// 'bar' === foo
					if (right.type === "JSReferenceIdentifier") {
						types.push({
							name: right.name,
							value: evaluator.getTypeFromEvaluatedNode(left),
						});
					}
					break;
				}

				case "!==": {
					// TODO add `typeof`
					if (left.type === "JSReferenceIdentifier") {
						types.push({
							name: left.name,
							value: new RefinedT(
								scope,
								left,
								evaluator.getTypeFromEvaluatedNode(left),
								evaluator.getTypeFromEvaluatedNode(right),
							),
						});
					}
					if (right.type === "JSReferenceIdentifier") {
						types.push({
							name: right.name,
							value: new RefinedT(
								scope,
								right,
								evaluator.getTypeFromEvaluatedNode(right),
								evaluator.getTypeFromEvaluatedNode(left),
							),
						});
					}
					return types;
				}

				case "instanceof":
					return [];

				default:
					throw new Error("Unknown JSBinaryExpression operator");
			}
			break;
		}

		case "JSLogicalExpression":
			switch (node.operator) {
				case "||": {
					const leftMap = typesToMap(genTypes(node.left, scope));
					const rightMap = typesToMap(genTypes(node.right, scope));
					const names = new Set([...leftMap.keys(), ...rightMap.keys()]);

					return Array.from(
						names,
						(name: string): TypeDefinition => {
							const left = leftMap.get(name);
							const right = rightMap.get(name);

							let type;

							if (left === undefined) {
								type = right;
							} else if (right === undefined) {
								type = left;
							} else {
								type = new UnionT(scope, undefined, [left, right]);
							}

							if (type === undefined) {
								throw new Error("Expected type");
							}

							return {
								name,
								value: type,
							};
						},
					);
				}

				case "&&":
					return [...genTypes(node.left, scope), ...genTypes(node.right, scope)];
			}
	}

	return types;
}

export default function refine(
	test: AnyNode,
	outerScope: Scope,
	hasAlternate: boolean,
): {
	consequent: Scope;
	alternate: Scope;
} {
	const consequent = outerScope.fork();
	const alternate = outerScope.fork();

	const rawTestTypes = genTypes(test, outerScope);

	const testTypes: ExtendedMap<string, Array<T>> = new ExtendedMap(
		"testTypes",
		() => [],
	);
	for (const {name, value} of rawTestTypes) {
		const types = testTypes.assert(name);
		types.push(value);
	}

	for (const [name, types] of testTypes) {
		// Build up the type in the case it's been refined to multiple values
		const type =
			types.length === 1 ? types[0] : new UnionT(outerScope, undefined, types);

		// Set type on `consequent`
		consequent.addBinding(name, type);

		// Remove type from '`alternate`
		if (hasAlternate) {
			const binding = outerScope.getBindingAssert(name);
			const opposite = new RefinedT(outerScope, type.originNode, binding, type);
			alternate.addBinding(name, opposite);
		}
	}

	// TODO, get binding refinements that were made inside
	return {consequent, alternate};
}
