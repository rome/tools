import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {hasJSXAttribute} from "@internal/js-ast-utils";
import {isJSXDOMElement} from "@internal/js-ast-utils/isJSXDOMElement";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import hasHTMLAttribute from "@internal/js-ast-utils/hasHTMLAttribute";

export default createLintVisitor({
	name: "a11y/useKeyWithClickEvents",
	enter(path) {
		const {node} = path;

		if (
			isJSXDOMElement(node) &&
			hasJSXAttribute(node, "onClick") &&
			!(hasJSXAttribute(node, "onKeyUp") ||
			hasJSXAttribute(node, "onKeyDown") ||
			hasJSXAttribute(node, "onKeyPress"))
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.A11_Y_USE_KEY_WITH_CLICK_EVENTS,
			);
		} else if (
			isHTMLElement(node) &&
			hasHTMLAttribute(node, "onclick") &&
			!(hasHTMLAttribute(node, "onkeyup") ||
			hasHTMLAttribute(node, "onkeydown") ||
			hasHTMLAttribute(node, "onkeypress"))
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.A11_Y_USE_KEY_WITH_CLICK_EVENTS,
			);
		}

		return signals.retain;
	},
});
