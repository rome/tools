/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	CompilerContext,
	Scope,
	createVisitor,
	signals,
} from "@internal/compiler";
import {
	AnyJSExpression,
	AnyJSStatement,
	AnyNode,
	JSCallExpression,
	JSClassDeclaration,
	JSClassExpression,
	JSClassMethod,
	JSFunctionHead,
	jsArrowFunctionExpression,
	jsBindingIdentifier,
	jsBlockStatement,
	jsCallExpression,
	jsClassMethod,
	jsClassPropertyMeta,
	jsExpressionStatement,
	jsFunctionHead,
	jsIdentifier,
	jsReferenceIdentifier,
	jsReturnStatement,
	jsSequenceExpression,
	jsStaticPropertyKey,
} from "@internal/ast";
import {template} from "@internal/js-ast-utils";

function hasClassProps(node: JSClassDeclaration | JSClassExpression): boolean {
	for (const bodyNode of node.meta.body) {
		if (bodyNode.type === "JSClassProperty") {
			return true;
		}
	}

	return false;
}

function createConstructor(
	rest: undefined | JSFunctionHead["rest"],
	body: Array<AnyJSStatement>,
) {
	return jsClassMethod.create({
		kind: "constructor",
		key: jsStaticPropertyKey.quick(jsIdentifier.quick("constructor")),
		meta: jsClassPropertyMeta.create({}),
		head: jsFunctionHead.create({params: [], rest}),
		body: jsBlockStatement.create({body}),
	});
}

function toExpressionStatements(
	expressions: Array<AnyJSExpression>,
): Array<AnyJSStatement> {
	return expressions.map((expr) => {
		return jsExpressionStatement.create({expression: expr});
	});
}

function isSuperCall(node: AnyNode): node is JSCallExpression {
	return node.type === "JSCallExpression" && node.callee.type === "JSSuper";
}

function transformClass(
	node: JSClassDeclaration,
	scope: Scope,
	context: CompilerContext,
): {
	newClass: JSClassDeclaration;
	className: string;
	declarations: Array<AnyNode>;
} {
	const bodyReplacements: Array<AnyJSStatement> = [];
	const constructorAssignments: Array<AnyJSExpression> = [];
	const className: string =
		node.id === undefined ? scope.generateUid("class") : node.id.name;

	let _constructor: undefined | JSClassMethod = undefined;
	const filteredClassBody = [];
	for (const bodyNode of node.meta.body) {
		if (bodyNode.type === "JSClassMethod" && bodyNode.kind === "constructor") {
			_constructor = bodyNode;
			continue;
		}

		if (bodyNode.type === "JSClassProperty") {
			if (bodyNode.value === undefined) {
				continue;
			}

			if (bodyNode.key.type === "JSComputedPropertyKey") {
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
									return signals.retain;
								}

								if (
									isSuperCall(node) &&
									path.parent.type !== "JSExpressionStatement"
								) {
									visited.add(node);

									// TODO retain proper value of super()
									return signals.replace(
										jsSequenceExpression.create({
											expressions: [node, ...constructorAssignments],
										}),
									);
								}

								if (
									node.type === "JSExpressionStatement" &&
									isSuperCall(node.expression)
								) {
									visited.add(node);

									return signals.replace(
										([node, ...toExpressionStatements(constructorAssignments)] as Array<AnyNode>),
									);
								}

								return signals.retain;
							},
						},
					],
				);
				_constructor = jsClassMethod.assert(reducedConstructor);
			} else {
				// create new constructor with a super() call and assignments
				_constructor = createConstructor(
					jsBindingIdentifier.quick("args"),
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

	const newClass: JSClassDeclaration = {
		...node,
		id: node.id !== undefined && node.id.name === className
			? node.id
			: jsBindingIdentifier.create({
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

export default createVisitor({
	name: "classProperties",
	enter(path) {
		const {node, scope, context} = path;

		// correctly replace an export class with the class node then append the declarations
		if (
			(node.type === "JSExportLocalDeclaration" ||
			node.type === "JSExportDefaultDeclaration") &&
			node.declaration !== undefined &&
			node.declaration.type === "JSClassDeclaration" &&
			hasClassProps(node.declaration)
		) {
			const {newClass, declarations} = transformClass(
				node.declaration,
				scope,
				context,
			);
			return signals.replace([
				{
					...node,
					declaration: newClass,
				},
				...declarations,
			]);
		}

		// turn a class expression into an IIFE that returns a class declaration
		if (node.type === "JSClassExpression" && hasClassProps(node)) {
			const className =
				node.id === undefined ? scope.generateUid("class") : node.id.name;

			return signals.replace(
				jsCallExpression.create({
					callee: jsArrowFunctionExpression.create({
						head: jsFunctionHead.quick([]),
						body: jsBlockStatement.create({
							body: [
								{
									...node,
									type: "JSClassDeclaration",
									id: jsBindingIdentifier.quick(className),
								},
								jsReturnStatement.create({
									argument: jsReferenceIdentifier.quick(className),
								}),
							],
						}),
					}),
					arguments: [],
				}),
			);
		}

		if (node.type === "JSClassDeclaration" && hasClassProps(node)) {
			const {newClass, declarations} = transformClass(node, scope, context);
			return signals.replace([newClass, ...declarations]);
		}

		return signals.retain;
	},
});
