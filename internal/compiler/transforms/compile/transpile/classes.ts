/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	CompilerContext,
	Path,
	createVisitor,
	signals,
} from "@internal/compiler";
import {
	AnyJSStatement,
	AnyNode,
	JSClassDeclaration,
	JSClassExpression,
	JSFunctionDeclaration,
	JSReferenceIdentifier,
	jsArrowFunctionExpression,
	jsBindingIdentifier,
	jsBlockStatement,
	jsCallExpression,
	jsClassDeclaration,
	jsClassMethod,
	jsFunctionDeclaration,
	jsFunctionExpression,
	jsFunctionHead,
	jsIdentifier,
	jsMemberExpression,
	jsReferenceIdentifier,
	jsReturnStatement,
	jsStaticMemberProperty,
	jsThisExpression,
} from "@internal/ast";
import {template} from "@internal/js-ast-utils";
import {descriptions} from "@internal/diagnostics";

function transformClass(
	node: JSClassDeclaration | JSClassExpression,
	path: Path,
	context: CompilerContext,
): {
	_constructor: JSFunctionDeclaration;
	prependDeclarations: Array<AnyJSStatement>;
	declarations: Array<AnyJSStatement>;
} {
	const {scope} = path;

	// declarations that we want to append and prepend, these include inheritance setup, method assignment, and other declarations
	const prependDeclarations = [];
	const declarations = [];

	// if the superClass is a global variable or a complex expression, then we should execute it once before the function is evaluated to ensure correct execution semantics
	let superClassRef: undefined | JSReferenceIdentifier;
	const {superClass} = node.meta;
	if (superClass !== undefined) {
		if (
			superClass.type === "JSReferenceIdentifier" &&
			scope.hasBinding(superClass.name)
		) {
			superClassRef = superClass;
		} else {
			superClassRef = jsReferenceIdentifier.create({
				name: scope.generateUid("superClass"),
			});
			prependDeclarations.push(
				template.statement`const ${superClassRef} = ${superClass};`,
			);
		}
	}

	// get the class name, if there's no class id then generate a new name
	const className: string =
		node.id === undefined ? scope.generateUid("class") : node.id.name;

	// push on the superClass setup
	if (superClass !== undefined) {
		if (superClassRef === undefined) {
			throw new Error("Impossible");
		}

		// inherit static properties

		// technically this isn't correct, the fully spec compliant version is Object.setPrototypeOf(Class, JSSuperClass);
		declarations.push(
			template.statement`Object.assign(${className}, ${superClassRef});`,
		);

		// inherit prototype
		declarations.push(
			template.statement`${className}.prototype = Object.create(${superClassRef} && ${superClassRef}.prototype);`,
		);

		// set correct prototype.constructor
		declarations.push(
			template.statement`${className}.prototype.constructor = ${className};`,
		);

		// some weird property the old babel transform apparently adds, TODO: check the actual usage of this
		declarations.push(
			template.statement`${className}.__superConstructor__ = ${superClassRef};`,
		);
	}

	const newNode = jsClassDeclaration.assert(
		path.reduceNode({
			name: "classesSuperTransform",
			enter(path) {
				if (superClassRef === undefined) {
					throw new Error("Impossible");
				}

				const {node} = path;

				// TODO correctly support super() by using return value
				if (node.type === "JSCallExpression" && node.callee.type === "JSSuper") {
					// replace super(...args); with JSSuper.call(this, ...args);
					return signals.replace(
						jsCallExpression.create({
							callee: jsMemberExpression.create({
								object: superClassRef,
								property: jsStaticMemberProperty.quick(
									jsIdentifier.quick("call"),
								),
							}),
							arguments: [jsThisExpression.create({}), ...node.arguments],
						}),
					);
				}

				// TODO super.foo
				if (node.type === "JSMemberExpression" && node.object.type === "JSSuper") {
					const jsClassMethod2 = path.findAncestry((path) =>
						path.node.type === "JSClassMethod"
					);
					if (jsClassMethod2 === undefined) {
						throw new Error("Expected to find class method here");
					}
					const isStatic = jsClassMethod.assert(jsClassMethod2.node).meta.static;

					const {property} = node;

					if (isStatic) {
						return signals.replace(
							jsMemberExpression.create({
								object: superClassRef,
								property,
							}),
						);
					}

					const superProtoRef = jsMemberExpression.create({
						object: superClassRef,
						property: jsStaticMemberProperty.quick(
							jsIdentifier.quick("prototype"),
						),
					});
					return signals.replace(
						jsMemberExpression.create({
							object: superProtoRef,
							property,
						}),
					);
				}

				// super.foo();
				if (
					node.type === "JSCallExpression" &&
					node.callee.type === "JSMemberExpression" &&
					node.callee.object.type === "JSSuper"
				) {
					const jsClassMethod2 = path.findAncestry((path) =>
						path.node.type === "JSClassMethod"
					);
					if (jsClassMethod2 === undefined) {
						throw new Error("Expected to find class method here");
					}
					const isStatic = jsClassMethod.assert(jsClassMethod2.node).meta.static;

					const args = node.arguments;
					const {property} = node.callee;

					// for static methods replace `super.foo(...args);` with `Super.foo.call(Class, ...args);`
					if (isStatic) {
						let methodRef;
						methodRef = jsMemberExpression.create({
							object: superClassRef,
							property,
						});
						return signals.replace(
							jsCallExpression.create({
								callee: jsMemberExpression.create({
									object: methodRef,
									property: jsStaticMemberProperty.quick(
										jsIdentifier.quick("call"),
									),
								}),
								arguments: [jsReferenceIdentifier.quick(className), ...args],
							}),
						);
					}

					// for instance methods replace `super.foo(...args)` with `Super.prototype.call(this, ...args)`
					let methodRef;
					let prototypeRef = jsMemberExpression.create({
						object: superClassRef,
						property: jsStaticMemberProperty.quick(
							jsIdentifier.quick("prototype"),
						),
					});
					methodRef = jsMemberExpression.create({
						object: prototypeRef,
						property,
					});
					return signals.replace(
						jsCallExpression.create({
							callee: jsMemberExpression.create({
								object: methodRef,
								property: jsStaticMemberProperty.quick(
									jsIdentifier.quick("call"),
								),
							}),
							arguments: [jsThisExpression.create({}), ...args],
						}),
					);
				}

				// TODO break when inside of functions
				return signals.retain;
			},
		}),
	);

	// setup method declarations
	let constructorMethod = undefined;
	for (const bodyNode of newNode.meta.body) {
		if (bodyNode.type !== "JSClassMethod") {
			context.addNodeDiagnostic(
				bodyNode,
				descriptions.COMPILER.CLASSES_UNSUPPORTED,
			);
			continue;
		}

		// save the constructor if this is it, we'll process this later
		if (bodyNode.kind === "constructor") {
			constructorMethod = bodyNode;
		}

		if (bodyNode.kind === "method") {
			// create the function expression to represent this method
			const functionNode = jsFunctionExpression.create({
				head: bodyNode.head,
				body: bodyNode.body,
			});

			// create the target node, for static methods this will be the base class, otherwise it's the prototype
			let target;
			if (bodyNode.meta.static === true) {
				target = jsIdentifier.quick(className);
			} else {
				target = template.expression`${className}.prototype`;
			}

			// use computed properties for computed methods
			if (bodyNode.key.type === "JSComputedPropertyKey") {
				declarations.push(
					template.statement`${target}[${bodyNode.key.value}] = ${functionNode}`,
				);
			} else {
				declarations.push(
					template.statement`${target}.${bodyNode.key.value} = ${functionNode}`,
				);
			}
		}
	}

	// create the constructor method
	let _constructor: JSFunctionDeclaration;
	if (constructorMethod === undefined) {
		if (superClassRef === undefined) {
			_constructor = jsFunctionDeclaration.assert(
				template.statement`function ${className}() {}`,
			);
		} else {
			_constructor = jsFunctionDeclaration.assert(
				template.statement`function ${className}(...args) {${superClassRef}.apply(this, args);}`,
			);
		}
	} else {
		_constructor = jsFunctionDeclaration.create({
			id: jsBindingIdentifier.quick(className),
			head: constructorMethod.head,
			body: constructorMethod.body,
		});
	}

	return {_constructor, prependDeclarations, declarations};
}

export default createVisitor({
	name: "classes",
	enter(path) {
		const {node, scope, context} = path;

		// correctly replace an export class with the class node then append the declarations
		if (
			(node.type === "JSExportLocalDeclaration" ||
			node.type === "JSExportDefaultDeclaration") &&
			node.declaration !== undefined &&
			node.declaration.type === "JSClassDeclaration"
		) {
			const {_constructor, declarations, prependDeclarations} = transformClass(
				node.declaration,
				path.getChildPath("declaration"),
				context,
			);
			const nodes: Array<AnyNode> = [
				...prependDeclarations,
				{
					...node,
					declaration: _constructor,
				},
				...declarations,
			];
			return signals.replace(nodes);
		}

		if (node.type === "JSClassDeclaration") {
			const {_constructor, prependDeclarations, declarations} = transformClass(
				node,
				path,
				context,
			);
			return signals.replace([
				...prependDeclarations,
				_constructor,
				...declarations,
			]);
		}

		// turn a class expression into an IIFE that returns a class declaration
		if (node.type === "JSClassExpression") {
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

		return signals.retain;
	},
});
