import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {
	doesNodeMatchPattern,
	getJSXAttribute,
	hasJSXAttribute,
} from "@romejs/js-ast-utils";
import {JSXAttribute} from "@romejs/ast";

function inClassComponent(path: Path): boolean {
	return (
		path.findAncestry(({node}) =>
			node.type === "JSClassMethod" &&
			node.key.type === "JSStaticPropertyKey" &&
			node.key.value.type === "JSIdentifier"
		) !== undefined
	);
}

function containsStringLiteral(attribute: JSXAttribute): boolean {
	return attribute.value?.type === "JSStringLiteral";
}

function containsStringContainer(attribute: JSXAttribute): boolean {
	if (attribute.value?.type !== "JSXExpressionContainer") {
		return false;
	}

	return (
		(attribute.value?.expression).type === "JSStringLiteral" ||
		(attribute.value?.expression).type === "JSTemplateLiteral"
	);
}

export default {
	name: "reactNoStringRefs",
	enter(path: Path): TransformExitResult {
		const {context, node} = path;

		if (inClassComponent(path) && doesNodeMatchPattern(node, "this.refs")) {
			context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_NO_STRING_REFS("<emphasis>this.refs</emphasis>"),
			);
		}

		if (node.type === "JSXElement" && hasJSXAttribute(node, "ref")) {
			const ref = getJSXAttribute(node, "ref");

			if (ref === undefined) {
				return node;
			}

			if (containsStringLiteral(ref) || containsStringContainer(ref)) {
				context.addNodeDiagnostic(
					ref,
					descriptions.LINT.REACT_NO_STRING_REFS(
						"string literals in ref attributes",
					),
				);
			}
		}

		return node;
	},
};
