import {descriptions} from "@internal/diagnostics";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isJSXElement,
} from "@internal/js-ast-utils";
import {createLintVisitor, signals} from "@internal/compiler";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import hasHTMLAttribute from "@internal/js-ast-utils/hasHTMLAttribute";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

export default createLintVisitor({
	name: "a11y/noOnChange",
	enter(path) {
		const {context, node} = path;
		if (isHTMLElement(node)) {
			if (node.name.name === "select" || node.name.name === "option") {
				if (
					hasHTMLAttribute(node, "onchange") &&
					!hasHTMLAttribute(node, "onblur")
				) {
					context.addNodeDiagnostic(
						getHTMLAttribute(node, "onchange"),
						descriptions.LINT.A11_Y_NO_ON_CHANGE("onblur", "onchange"),
					);
				}
			}
		} else {
			if (!(isJSXElement(node, "select") || isJSXElement(node, "option"))) {
				return signals.retain;
			}

			if (!hasJSXAttribute(node, "onChange")) {
				return signals.retain;
			}

			if (hasJSXAttribute(node, "onBlur")) {
				return signals.retain;
			}

			context.addNodeDiagnostic(
				getJSXAttribute(node, "onChange"),
				descriptions.LINT.A11_Y_NO_ON_CHANGE("onBlur", "onChange"),
			);
		}

		return signals.retain;
	},
});
