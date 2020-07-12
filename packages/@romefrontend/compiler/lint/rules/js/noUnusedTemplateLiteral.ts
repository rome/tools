import {Path, TransformExitResult} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {JSTemplateElement, jsStringLiteral} from "@romefrontend/ast";

function containsSpecialCharacters(node: JSTemplateElement): boolean {
	return /['"\n]/.test(node.raw);
}

export default {
	name: "jsNoUnusedTemplateLiteral",
	enter(path: Path): TransformExitResult {
		const {context, node} = path;

		if (
			node.type === "JSTemplateLiteral" &&
			path.parent.type !== "JSTaggedTemplateExpression" &&
			node.expressions.length === 0 &&
			!node.quasis.some(containsSpecialCharacters)
		) {
			return context.addFixableDiagnostic(
				{
					old: node,
					fixed: jsStringLiteral.quick(
						node.quasis.map((node) => node.cooked).join(),
					),
				},
				descriptions.LINT.JS_NO_UNUSED_TEMPLATE_LITERAL,
			);
		}

		return node;
	},
};
