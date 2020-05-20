import {Path, TransformExitResult} from "@romejs/js-compiler";
import {descriptions} from "@romejs/diagnostics";
import {AnyNode} from "@romejs/ast";
import {
	doesNodeMatchPattern,
	getJSXAttribute,
	hasJSXAttribute,
	isJSXElement,
} from "@romejs/js-ast-utils";

const BUTTON_TYPE_REGEX = /^(reset)|(submit)|(button)$/;

function createElementMissingType(node: AnyNode) {
	if (node.type !== "JSCallExpression") {
		return false;
	}
	if (
		(doesNodeMatchPattern(node.callee, "React.createElement") ||
		doesNodeMatchPattern(node.callee, "createElement")) &&
		node.arguments[0].type === "JSStringLiteral" &&
		node.arguments[0].value === "button" &&
		node.arguments[1].type === "JSObjectExpression" &&
		!node.arguments[1].properties.find((prop) =>
			prop.type === "JSObjectProperty" &&
			prop.key.value.type === "JSIdentifier" &&
			prop.key.value.name === "type" &&
			prop.value.type === "JSStringLiteral" &&
			BUTTON_TYPE_REGEX.test(prop.value.value)
		)
	) {
		return true;
	}
	return false;
}

function jsxMissingType(node: AnyNode) {
	if (!isJSXElement(node, "button")) {
		return false;
	}
	if (!hasJSXAttribute(node, "type")) {
		return true;
	}
	const valueNode = getJSXAttribute(node, "type")?.value;
	if (
		valueNode?.type === "JSStringLiteral" &&
		!BUTTON_TYPE_REGEX.test(valueNode.value)
	) {
		return true;
	}
	return false;
}

export default {
	name: "buttonHasType",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (createElementMissingType(node) || jsxMissingType(node)) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_BUTTON_HAS_TYPE,
			);
		}

		return node;
	},
};
