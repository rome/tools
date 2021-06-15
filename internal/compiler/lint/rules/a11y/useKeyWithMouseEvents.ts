import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {getJSXAttribute, hasJSXAttribute} from "@internal/js-ast-utils";
import {isJSXDOMElement} from "@internal/js-ast-utils/isJSXDOMElement";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import hasHTMLAttribute from "@internal/js-ast-utils/hasHTMLAttribute";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

export default createLintVisitor({
	name: "a11y/useKeyWithMouseEvents",
	enter(path) {
		const {node} = path;

		if (isJSXDOMElement(node)) {
			if (
				hasJSXAttribute(node, "onMouseOver") &&
				!hasJSXAttribute(node, "onFocus")
			) {
				path.context.addNodeDiagnostic(
					getJSXAttribute(node, "onMouseOver"),
					descriptions.LINT.A11_Y_USE_KEY_WITH_MOUSE_EVENTS(
						"onMouseOver",
						"onFocus",
					),
				);
			}

			if (
				hasJSXAttribute(node, "onMouseOut") &&
				!hasJSXAttribute(node, "onBlur")
			) {
				path.context.addNodeDiagnostic(
					getJSXAttribute(node, "onMouseOut"),
					descriptions.LINT.A11_Y_USE_KEY_WITH_MOUSE_EVENTS(
						"onMouseOut",
						"onBlur",
					),
				);
			}
		} else if (isHTMLElement(node)) {
			if (
				hasHTMLAttribute(node, "onmouseover") &&
				!hasHTMLAttribute(node, "onfocus")
			) {
				path.context.addNodeDiagnostic(
					getHTMLAttribute(node, "onmouseover"),
					descriptions.LINT.A11_Y_USE_KEY_WITH_MOUSE_EVENTS(
						"onmouseover",
						"onfocus",
					),
				);
			}

			if (
				hasHTMLAttribute(node, "onmouseout") &&
				!hasHTMLAttribute(node, "onblur")
			) {
				path.context.addNodeDiagnostic(
					getHTMLAttribute(node, "onmouseout"),
					descriptions.LINT.A11_Y_USE_KEY_WITH_MOUSE_EVENTS(
						"onmouseout",
						"onblur",
					),
				);
			}
		}

		return signals.retain;
	},
});
