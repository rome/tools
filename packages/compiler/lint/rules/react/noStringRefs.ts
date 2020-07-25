import {Path, TransformExitResult} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {
	doesNodeMatchPattern,
	getJSXAttribute,
	hasJSXAttribute,
} from "@romefrontend/js-ast-utils";
import {JSXAttribute} from "@romefrontend/ast";
import {insideClassComponent} from "../../utils/react";
import {markup} from "@romefrontend/cli-layout";

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
	name: "react/noStringRefs",
	enter(path: Path): TransformExitResult {
		const {context, node} = path;

		if (insideClassComponent(path) && doesNodeMatchPattern(node, "this.refs")) {
			context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_NO_STRING_REFS(
					markup`<emphasis>this.refs</emphasis>`,
				),
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
						markup`string literals in ref attributes`,
					),
				);
			}
		}

		return node;
	},
};
