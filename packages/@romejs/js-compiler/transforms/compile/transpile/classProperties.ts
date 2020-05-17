/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {CompilerContext, Path, Scope} from "@romejs/js-compiler";
import {
	AnyExpression,
	AnyNode,
	AnyStatement,
	CallExpression,
	ClassDeclaration,
	ClassExpression,
	ClassMethod,
	FunctionHead,
	arrowFunctionExpression,
	bindingIdentifier,
	blockStatement,
	callExpression,
	classMethod,
	classPropertyMeta,
	expressionStatement,
	functionHead,
	identifier,
	referenceIdentifier,
	returnStatement,
	sequenceExpression,
	staticPropertyKey,
} from "@romejs/js-ast";
import {template} from "@romejs/js-ast-utils";

function hasClassProps(node: ClassDeclaration | ClassExpression): boolean {
	for (const bodyNode of node.meta.body) {
		if (bodyNode.type === "ClassProperty") {
			return true;
		}
	}

	return false;
}

function createConstructor(
	rest: undefined | FunctionHead["rest"],
	body: Array<AnyStatement>,
) {
	return classMethod.create({
		kind: "constructor",
		key: staticPropertyKey.quick(identifier.quick("constructor")),
		meta: classPropertyMeta.create({}),
		head: functionHead.create({params: [], rest}),
		body: blockStatement.create({body}),
	});
}

function toExpressionStatements(
	expressions: Array<AnyExpression>,
): Array<AnyStatement> {
	return expressions.map((expr) => {
		return expressionStatement.create({expression: expr});
	});
}

function isSuperCall(node: AnyNode): node is CallExpression {
	return node.type === "CallExpression" && node.callee.type === "Super";
}

function transformClass(
	node: ClassDeclaration,
	scope: Scope,
	context: CompilerContext,
): {
	newClass: ClassDeclaration;
	className: string;
	declarations: Array<AnyNode>;
} {
	const bodyReplacements: Array<AnyStatement> = [];
	const constructorAssignments: Array<AnyExpression> = [];
	const className: string =
		node.id === undefined ? scope.generateUid("class") : node.id.name;

	let _constructor: undefined | ClassMethod = undefined;
	const filteredClassBody = [];
	for (const bodyNode of node.meta.body) {
		if (bodyNode.type === "ClassMethod" && bodyNode.kind === "constructor") {
			_constructor = bodyNode;
			continue;
		}

		if (bodyNode.type === "ClassProperty") {
			if (bodyNode.value === undefined) {
				continue;
			}

			if (bodyNode.key.type === "ComputedPropertyKey") {
				if (bodyNode.meta.static === true) {
					bodyReplacements.push(
						template.statement`${className}[${bodyNode.key.value}] = ${bodyNode.value};`,
					);
				} else {
					constructorAssignments.push(
						template.expression`this[${bodyNode.key.value}] = ${bodyNode.value};`,
					);
				}
			} else {
				if (bodyNode.meta.static === true) {
					bodyReplacements.push(
						template.statement`${className}.${bodyNode.key.value} = ${bodyNode.value};`,
					);
				} else {
					constructorAssignments.push(
						template.expression`this.${bodyNode.key.value} = ${bodyNode.value};`,
					);
				}
			}
		} else {
			filteredClassBody.push(bodyNode);
		}
	}

	if (constructorAssignments.length) {
		if (node.meta.superClass !== undefined) {
			if (_constructor) {
				const visited = new Set();

				// find super() and insert assignments
				const reducedConstructor = context.reduce(
					_constructor,
					[
						{
							name: "classPropertiesInjector",
							enter(path) {
								const {node} = path;

								if (visited.has(node)) {
									return node;
								}

								if (
									isSuperCall(node) &&
									path.parent.type !== "ExpressionStatement"
								) {
									visited.add(node);

									// TODO retain proper value of super()
									return sequenceExpression.create({
										expressions: [node, ...constructorAssignments],
									});
								}

								if (
									node.type === "ExpressionStatement" &&
									isSuperCall(node.expression)
								) {
									visited.add(node);

									return ([
										node,
										...toExpressionStatements(constructorAssignments),
									] as Array<AnyNode>);
								}

								return node;
							},
						},
					],
				);
				_constructor = classMethod.assert(reducedConstructor);
			} else {
				// create new constructor with a super() call and assignments
				_constructor = createConstructor(
					bindingIdentifier.quick("args"),
					[
						template.statement`super(...args);`,
						...toExpressionStatements(constructorAssignments),
					],
				);
			}
		} else {
			if (_constructor) {
				// add assignments to end of constructor
				_constructor = {
					..._constructor,
					body: {
						..._constructor.body,
						body: [
							...toExpressionStatements(constructorAssignments),
							..._constructor.body.body,
						],
					},
				};
			} else {
				// create new constructor with just the assignments
				_constructor = createConstructor(
					undefined,
					toExpressionStatements(constructorAssignments),
				);
			}
		}
	}

	if (_constructor !== undefined) {
		filteredClassBody.unshift(_constructor);
	}

	const newClass: ClassDeclaration = {
		...node,
		id: node.id !== undefined && node.id.name === className
			? node.id
			: bindingIdentifier.create({
					name: className,
				}),
		meta: {
			...node.meta,
			body: filteredClassBody,
		},
	};

	return {
		newClass,
		className,
		declarations: bodyReplacements,
	};
}

export default {
	name: "classProperties",
	enter(path: Path) {
		const {node, scope, context} = path;

		// correctly replace an export class with the class node then append the declarations
		if (
			(node.type === "ExportLocalDeclaration" ||
			node.type === "ExportDefaultDeclaration") &&
			node.declaration !== undefined &&
			node.declaration.type === "ClassDeclaration" &&
			hasClassProps(node.declaration)
		) {
			const {newClass, declarations} = transformClass(
				node.declaration,
				scope,
				context,
			);
			return ([
				{
					...node,
					declaration: newClass,
				},
				...declarations,
			] as Array<AnyNode>);
		}

		// turn a class expression into an IIFE that returns a class declaration
		if (node.type === "ClassExpression" && hasClassProps(node)) {
			const className =
				node.id === undefined ? scope.generateUid("class") : node.id.name;

			return callExpression.create({
				callee: arrowFunctionExpression.create({
					head: functionHead.quick([]),
					body: blockStatement.create({
						body: [
							{
								...node,
								type: "ClassDeclaration",
								id: bindingIdentifier.quick(className),
							},
							returnStatement.create({
								argument: referenceIdentifier.quick(className),
							}),
						],
					}),
				}),
				arguments: [],
			});
		}

		if (node.type === "ClassDeclaration" && hasClassProps(node)) {
			const {newClass, declarations} = transformClass(node, scope, context);
			return ([newClass, ...declarations] as Array<AnyNode>);
		}

		return node;
	},
};
