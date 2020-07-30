import {AnyJSExpression, JSXAttribute, JSXElement} from "@internal/ast";

function isEmptyAttributeValue(
	node: NonNullable<JSXAttribute["value"]> | AnyJSExpression,
): boolean {
	switch (node.type) {
		case "JSStringLiteral":
			return node.value === "";

		case "JSXExpressionContainer":
			return isEmptyAttributeValue(node.expression);

		case "JSReferenceIdentifier":
			return node.name === "undefined";

		case "JSXEmptyExpression":
			return true;

		default:
			return false;
	}
}

export function getJSXAttribute(
	tag: JSXElement,
	name: string,
	allowEmpty: boolean = false,
): JSXAttribute | undefined {
	for (const attr of tag.attributes) {
		if (attr.type === "JSXAttribute" && attr.name.name === name) {
			const {value} = attr;

			if (value !== undefined && !allowEmpty && isEmptyAttributeValue(value)) {
				return undefined;
			}

			return attr;
		}
	}
	return undefined;
}
