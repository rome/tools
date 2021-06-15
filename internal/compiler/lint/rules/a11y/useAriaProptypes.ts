import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {isEmptyTemplateLiteral} from "@internal/js-ast-utils";
import {
	ARIAProperty,
	ARIAPropertyDefinition,
	ariaPropsMap,
} from "@internal/compiler/lint/utils/aria";

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
				return attribute.values.some((token) => {
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

const ARIA_PREFIX = "aria-";

export default createLintVisitor({
	name: "a11y/useAriaProptypes",
	enter(path) {
		const {node} = path;
		if (node.type === "HTMLAttribute") {
			if (node.name.name.startsWith(ARIA_PREFIX)) {
				if (node.value) {
					const ariaAttribute = ariaPropsMap.get(node.name.name as ARIAProperty);
					if (ariaAttribute) {
						if (
							node.value.value === "" ||
							!isCorrectValue(ariaAttribute, node.value.value)
						) {
							path.context.addNodeDiagnostic(
								node,
								descriptions.LINT.A11_Y_USE_ARIA_PROPTYPES(
									node.name.name,
									ariaAttribute.values,
								),
							);
						}
					}
				}
			}
		} else if (node.type === "JSXAttribute") {
			if (
				node.name.type === "JSXIdentifier" &&
				node.name.name.startsWith(ARIA_PREFIX)
			) {
				if (node.value) {
					const ariaAttribute = ariaPropsMap.get(node.name.name as ARIAProperty);
					if (ariaAttribute) {
						if (node.value.type === "JSStringLiteral") {
							if (
								node.value.value === "" ||
								!isCorrectValue(ariaAttribute, node.value.value)
							) {
								path.context.addNodeDiagnostic(
									node,
									descriptions.LINT.A11_Y_USE_ARIA_PROPTYPES(
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
										descriptions.LINT.A11_Y_USE_ARIA_PROPTYPES(
											node.name.name,
											ariaAttribute.values,
										),
									);
								}
							} else if (expression.type === "JSTemplateLiteral") {
								if (isEmptyTemplateLiteral(expression)) {
									path.context.addNodeDiagnostic(
										node,
										descriptions.LINT.A11_Y_USE_ARIA_PROPTYPES(
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

		return signals.retain;
	},
});
