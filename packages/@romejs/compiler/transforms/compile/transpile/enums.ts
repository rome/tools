import {Path} from "@romejs/compiler";
import {
	AnyJSExpression,
	JSIdentifier,
	JSNumericLiteral,
	JSObjectProperties,
	JSObjectProperty,
	JSPrivateName,
	JSStringLiteral,
	TSEnumMember,
	jsBindingIdentifier,
	jsIdentifier,
	jsNumericLiteral,
	jsObjectExpression,
	jsObjectProperty,
	jsStaticPropertyKey,
	jsStringLiteral,
	jsVariableDeclaration,
	jsVariableDeclarator,
} from "@romejs/ast";

function getMemberName(member: TSEnumMember): string {
	return member.id.type === "JSIdentifier" ? member.id.name : member.id.value;
}

function createMember(
	key:
		| string
		| number
		| JSStringLiteral
		| JSIdentifier
		| JSNumericLiteral
		| JSPrivateName,
	value: string | number | AnyJSExpression,
): JSObjectProperty {
	let keyNode;
	if (typeof key === "string") {
		keyNode = jsIdentifier.create({name: key});
	} else if (typeof key === "number") {
		keyNode = jsNumericLiteral.create({value: key});
	} else {
		keyNode = key;
	}

	let valueNode: AnyJSExpression;
	if (typeof value === "string") {
		valueNode = jsStringLiteral.create({value});
	} else if (typeof value === "number") {
		valueNode = jsNumericLiteral.create({value});
	} else {
		valueNode = value;
	}

	return jsObjectProperty.create({
		key: jsStaticPropertyKey.create({
			value: keyNode,
		}),
		value: valueNode,
	});
}

export default {
	name: "enums",
	enter(path: Path) {
		const {node} = path;

		let currentIndex = 0;

		if (node.type === "TSEnumDeclaration") {
			return jsVariableDeclaration.create({
				kind: "const",
				declarations: [
					jsVariableDeclarator.create({
						id: jsBindingIdentifier.create({
							name: node.id.name,
						}),
						init: jsObjectExpression.create({
							properties: node.members.reduce<JSObjectProperties>(
								(properties, member) => {
									properties.push(
										createMember(
											getMemberName(member),
											member.initializer || currentIndex,
										),
									);

									if (
										!member.initializer ||
										member.initializer.type === "JSNumericLiteral"
									) {
										properties.push(
											createMember(
												member.initializer || currentIndex,
												getMemberName(member),
											),
										);
									}

									if (
										member.initializer &&
										member.initializer.type === "JSNumericLiteral"
									) {
										currentIndex = member.initializer.value + 1;
									} else if (!member.initializer) {
										currentIndex++;
									}

									return properties;
								},
								[],
							),
						}),
					}),
				],
			});
		}

		return node;
	},
};
