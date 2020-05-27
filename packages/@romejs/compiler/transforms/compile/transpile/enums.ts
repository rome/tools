import {Path} from "@romejs/compiler";
import {
	AnyJSExpression,
	JSObjectProperties,
	JSObjectProperty,
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
	key: string | number,
	value: string | number | AnyJSExpression,
): JSObjectProperty {
	const keyNode =
		typeof key === "string"
			? jsIdentifier.create({
					name: key,
				})
			: jsNumericLiteral.create({
					value: key,
				});
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
								(properties, member, index) => {
									if (member.initializer) {
										properties.push(
											createMember(getMemberName(member), member.initializer),
										);
									} else {
										properties.push(
											createMember(index, getMemberName(member)),
											createMember(getMemberName(member), index),
										);
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
