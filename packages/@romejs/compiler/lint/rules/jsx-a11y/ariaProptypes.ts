import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {
	ARIAProperty,
	ARIAPropertyDefinition,
	ariaPropsMap,
} from "@romejs/compiler/lint/rules/ariaHelpers";
import isEmptyTemplateLiteral from "@romejs/js-ast-utils/isEmptyTemplateLiteral";

function isCorrectValue(
	attribute: ARIAPropertyDefinition,
	value: string | number | boolean,
): boolean {
	switch (attribute.type) {
		case "boolean": {
			if (typeof value === "string") {
				return value === "false" || value === "true";
			}
			return typeof value === "boolean";
		}

		case "id":
		case "string":
			return typeof value === "string";

		case "tristate": {
			if (typeof value === "string") {
				return value === "false" || value === "true" || value === "mixed";
			}
			if (typeof value === "boolean") {
				return Boolean(value);
			}
			return false;
		}
		case "token": {
			if (attribute.values) {
				const result = attribute.values.some((token) => {
					if (typeof token === "boolean") {
						if (typeof value === "string") {
							return value === "true" || value === "false";
						}
						return Boolean(value) === token;
					}
					return typeof value === "string"
						? value === token
						: `${value}` === token;
				});
				return result;
			}
			return false;
		}

		case "idlist": {
			return (
				typeof value === "string" &&
				value.split(" ").every((token) => typeof token === "string")
			);
		}

		case "integer":
		case "number":
			return typeof value !== "boolean" && isNaN(Number(value)) === false;

		case "tokenlist": {
			if (attribute.values) {
				return (
					typeof value === "string" &&
					value.split(" ").every((token) => {
						return (
							attribute.values &&
							attribute.values.indexOf(token.toLowerCase()) > -1
						);
					})
				);
			}
			return false;
		}

		default:
			return false;
	}
}

export default {
	name: "jsxA11YAriaProptypes",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (node.type === "JSXAttribute") {
			if (
				node.name.type === "JSXIdentifier" &&
				node.name.name.indexOf("aria-") === 0
			) {
				if (node.value) {
					const ariaAttribute = ariaPropsMap.get(
						(node.name.name as ARIAProperty),
					);
					if (ariaAttribute) {
						if (node.value.type === "JSStringLiteral") {
							if (
								node.value.value === "" ||
								!isCorrectValue(ariaAttribute, node.value.value)
							) {
								path.context.addNodeDiagnostic(
									node,
									descriptions.LINT.JSX_A11Y_ARIA_PROPTYPES(
										node.name.name,
										ariaAttribute.values,
									),
								);
							}
						}

						if (node.value.type === "JSXExpressionContainer") {
							const expression = node.value.expression;

							if (
								expression.type === "JSBooleanLiteral" ||
								expression.type === "JSNumericLiteral"
							) {
								if (!isCorrectValue(ariaAttribute, expression.value)) {
									path.context.addNodeDiagnostic(
										node,
										descriptions.LINT.JSX_A11Y_ARIA_PROPTYPES(
											node.name.name,
											ariaAttribute.values,
										),
									);
								}
							} else if (expression.type === "JSTemplateLiteral") {
								if (isEmptyTemplateLiteral(expression)) {
									path.context.addNodeDiagnostic(
										node,
										descriptions.LINT.JSX_A11Y_ARIA_PROPTYPES(
											node.name.name,
											ariaAttribute.values,
										),
									);
								}
							}
						}
					}
				}
			}
		}

		return node;
	},
};
