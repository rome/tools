import {descriptions} from "@internal/diagnostics";
import {AnyNode} from "@internal/ast";
import {createLintVisitor, signals} from "@internal/compiler";
import {getJSXAttribute, isJSXElement} from "@internal/js-ast-utils";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

function jsxImgRedundantAlt(node: AnyNode) {
	if (!isJSXElement(node, "img")) {
		return false;
	}

	const attr = getJSXAttribute(node, "alt");
	return (
		attr?.value?.type === "JSStringLiteral" &&
		/(image)|(picture)|(photo)/i.test(attr.value.value)
	);
}

export default createLintVisitor({
	name: "a11y/noRedundantAlt",

	enter(path) {
		const {node} = path;

		if (jsxImgRedundantAlt(node)) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.A11_Y_NO_REDUNDANT_ALT,
			);
		} else if (isHTMLElement(node)) {
			const attr = getHTMLAttribute(node, "alt");
			if (
				attr?.value?.value &&
				/(image)|(picture)|(photo)/i.test(attr?.value?.value)
			) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.A11_Y_NO_REDUNDANT_ALT,
				);
			}
		}

		return signals.retain;
	},
});
