/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	JSCallExpression,
	JSMemberExpression,
	JSObjectProperties,
	JSObjectProperty,
	JSReferenceIdentifier,
	JSStringLiteral,
	JSThisExpression,
	JSXAttribute,
	JSXElement,
	JSXExpressionContainer,
	JSXIdentifier,
	JSXNamespacedName,
	jsBooleanLiteral,
	jsCallExpression,
	jsComputedMemberProperty,
	jsComputedPropertyKey,
	jsIdentifier,
	jsMemberExpression,
	jsNullLiteral,
	jsObjectExpression,
	jsObjectProperty,
	jsReferenceIdentifier,
	jsSpreadElement,
	jsStaticMemberProperty,
	jsStaticPropertyKey,
	jsStringLiteral,
	jsThisExpression,
	jsxExpressionContainer,
	jsxIdentifier,
} from "@internal/ast";
import {Path, createVisitor, signals} from "@internal/compiler";
import {
	cleanJSXText,
	inheritLoc,
	isValidIdentifierName,
	template,
} from "@internal/js-ast-utils";
import {descriptions} from "@internal/diagnostics";

function convertJSXIdentifier(
	path: Path,
):
	| JSMemberExpression
	| JSThisExpression
	| JSStringLiteral
	| JSReferenceIdentifier {
	const {node} = path;

	if (node.type === "JSXReferenceIdentifier") {
		if (node.name === "this") {
			return jsThisExpression.create({});
		} else {
			return jsReferenceIdentifier.create(
				{
					name: node.name,
				},
				node,
			);
		}
	} else if (node.type === "JSXIdentifier") {
		return jsStringLiteral.quick(node.name);
	} else if (node.type === "JSXMemberExpression") {
		let prop = convertJSXIdentifier(path.getChildPath("property"));

		if (prop.type === "JSReferenceIdentifier") {
			return jsMemberExpression.create({
				object: convertJSXIdentifier(path.getChildPath("object")),
				property: jsStaticMemberProperty.quick(jsIdentifier.quick(prop.name)),
			});
		} else {
			return jsMemberExpression.create({
				object: convertJSXIdentifier(path.getChildPath("object")),
				property: jsComputedMemberProperty.quick(prop),
			});
		}
	} else {
		throw new Error(
			`Received a node of type ${node.type}, the only node types that should be in this position are JSXIdentifier and JSXMemberExpression`,
		);
	}
}

function convertAttributeValue(
	node: AnyJSExpression | JSXExpressionContainer,
): AnyJSExpression {
	if (node.type === "JSXExpressionContainer") {
		return node.expression;
	} else {
		return node;
	}
}

function extractName(node: JSXIdentifier | JSXNamespacedName): string {
	if (node.type === "JSXNamespacedName") {
		throw new Error("JSX is not XML blah blah blah");
	} else {
		return jsxIdentifier.assert(node).name;
	}
}

function convertAttribute(node: JSXAttribute): JSObjectProperty {
	let valueNode = convertAttributeValue(
		node.value ||
		jsBooleanLiteral.create({
			value: true,
		}),
	);
	if (
		valueNode.type === "JSStringLiteral" &&
		(!node.value || node.value.type !== "JSXExpressionContainer")
	) {
		valueNode = jsStringLiteral.create({
			value: valueNode.value.replace(/\n\s+/g, " "),
		});
	}

	const name = extractName(node.name);

	if (isValidIdentifierName(name)) {
		const nameNode = jsIdentifier.create({
			name,
			loc: inheritLoc(node),
		});

		return jsObjectProperty.create({
			key: jsStaticPropertyKey.quick(nameNode),
			value: valueNode,
		});
	} else {
		return jsObjectProperty.create({
			key: jsComputedPropertyKey.quick(jsStringLiteral.quick(name)),
			value: valueNode,
		});
	}
}

function pushProps(
	_props: JSObjectProperties,
	objs: Array<AnyJSExpression>,
): JSObjectProperties {
	if (!_props.length) {
		return _props;
	}

	objs.push(jsObjectExpression.create({properties: _props}));
	return [];
}

function buildOpeningElementAttributes(attribs: JSXElement["attributes"]) {
	let _props: JSObjectProperties = [];
	const objs: Array<AnyJSExpression> = [];

	for (const prop of attribs) {
		if (prop.type === "JSXSpreadAttribute") {
			_props = pushProps(_props, objs);
			objs.push(prop.argument);
		} else {
			_props.push(convertAttribute(prop));
		}
	}

	pushProps(_props, objs);

	let ret: AnyJSExpression;
	if (objs.length === 1) {
		// only one object
		ret = objs[0];
	} else {
		// looks like we have multiple objects
		if (objs[0].type !== "JSObjectExpression") {
			objs.unshift(jsObjectExpression.create({properties: []}));
		}

		// spread it
		ret = jsCallExpression.create({
			callee: template.expression`Object.assign`,
			arguments: objs,
		});
	}

	return ret;
}

function cleanJSXElementLiteralChild(value: string): undefined | JSStringLiteral {
	const str = cleanJSXText(value);
	if (str !== "") {
		return jsStringLiteral.quick(str);
	} else {
		return undefined;
	}
}

function buildChildren(
	children: JSXElement["children"],
): JSCallExpression["arguments"] {
	const elems: JSCallExpression["arguments"] = [];

	for (let child of children) {
		if (child.type === "JSXText") {
			const node = cleanJSXElementLiteralChild(child.value);
			if (node !== undefined) {
				elems.push(node);
			}
			continue;
		}

		if (child.type === "JSXExpressionContainer") {
			const {expression} = child;
			if (expression.type !== "JSXEmptyExpression") {
				elems.push(child.expression);
			}
			continue;
		}

		if (child.type === "JSXSpreadChild") {
			elems.push(jsSpreadElement.quick(child.expression));
			continue;
		}

		elems.push(child);
	}

	return elems;
}

export default createVisitor({
	name: "jsx",
	enter(path) {
		const {node, context, parent} = path;

		if (node.type === "JSXElement") {
			let type = convertJSXIdentifier(path.getChildPath("name"));

			if (node.name.type === "JSXNamespacedName") {
				// TODO better handle this
				context.addNodeDiagnostic(type, descriptions.COMPILER.JSX_NOT_XML);
			}

			let attribs: AnyJSExpression;
			if (node.attributes.length > 0) {
				attribs = buildOpeningElementAttributes(node.attributes);
			} else {
				attribs = jsNullLiteral.create({});
			}

			const call = jsCallExpression.create({
				callee: template.expression`React.createElement`,
				arguments: [type, attribs, ...buildChildren(node.children)],
			});

			// If we're a JSX element child then we need to be wrapped
			if (parent.type === "JSXElement") {
				return signals.replace(
					jsxExpressionContainer.create({
						expression: call,
					}),
				);
			} else {
				return signals.replace(call);
			}
		}

		if (node.type === "JSXFragment") {
			const type = template.expression`React.Fragment`;
			const attribs = template.expression`null`;
			return signals.replace(
				jsCallExpression.create({
					callee: template.expression`React.createElement`,
					arguments: [type, attribs, ...buildChildren(node.children)],
				}),
			);
		}

		return signals.retain;
	},
});
