import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {getJSXAttribute, hasJSXAttribute} from "@internal/js-ast-utils";

export default createVisitor({
	name: "jsx-a11y/useKeyWithMouseEvents",
	enter(path) {
		const {node} = path;

		if (node.type === "JSXElement") {
			if (
				hasJSXAttribute(node, "onMouseOver") &&
				!hasJSXAttribute(node, "onFocus")
			) {
				path.context.addNodeDiagnostic(
					getJSXAttribute(node, "onMouseOver"),
					descriptions.LINT.JSX_A11Y_MOUSE_EVENTS_HAVE_KEY_EVENTS(
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
					descriptions.LINT.JSX_A11Y_MOUSE_EVENTS_HAVE_KEY_EVENTS(
						"onMouseOut",
						"onBlur",
					),
				);
			}
		}

		return signals.retain;
	},
});
