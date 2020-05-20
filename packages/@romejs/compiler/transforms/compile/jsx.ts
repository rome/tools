/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	AnyNode,
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
	jsxElement,
	jsxExpressionContainer,
	jsxIdentifier,
	jsxNamespacedName,
} from "@romejs/ast";
import {Path} from "@romejs/compiler";
import {
	inheritLoc,
	isValidIdentifierName,
	template,
} from "@romejs/js-ast-utils";
import {descriptions} from "@romejs/diagnostics";

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

	while (attribs.length > 0) {
		const prop = attribs.shift()!;

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
	const lines = value.split(/\r\n|\n|\r/);

	let lastNonEmptyLine = 0;

	for (let i = 0; i < lines.length; i++) {
		if (lines[i].match(/[^ \t]/)) {
			lastNonEmptyLine = i;
		}
	}

	let str = "";

	for (let i = 0; i < lines.length; i++) {
		const line = lines[i];

		const isFirstLine = i === 0;
		const isLastLine = i === lines.length - 1;
		const isLastNonEmptyLine = i === lastNonEmptyLine;

		// replace rendered whitespace tabs with spaces
		let trimmedLine = line.replace(/\t/g, " ");

		// trim whitespace touching a newline
		if (!isFirstLine) {
			trimmedLine = trimmedLine.replace(/^[ ]+/, "");
		}

		// trim whitespace touching an endline
		if (!isLastLine) {
			trimmedLine = trimmedLine.replace(/[ ]+$/, "");
		}

		if (trimmedLine) {
			if (!isLastNonEmptyLine) {
				trimmedLine += " ";
			}

			str += trimmedLine;
		}
	}

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

export default {
	name: "jsx",
	enter(path: Path): AnyNode {
		const {node, context, parent} = path;

		if (jsxElement.is(node)) {
			let type = convertJSXIdentifier(path.getChildPath("name"));

			if (jsxNamespacedName.is(node.name)) {
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
			if (jsxElement.is(parent)) {
				return jsxExpressionContainer.create({
					expression: call,
				});
			} else {
				return call;
			}
		}

		if (node.type === "JSXFragment") {
			const type = template.expression`React.Fragment`;
			const attribs = template.expression`null`;
			return jsCallExpression.create({
				callee: template.expression`React.createElement`,
				arguments: [type, attribs, ...buildChildren(node.children)],
			});
		}

		return node;
	},
};
