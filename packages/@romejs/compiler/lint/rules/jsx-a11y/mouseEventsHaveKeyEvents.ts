import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {getJSXAttribute, hasJSXAttribute} from "@romejs/js-ast-utils";

export default {
	name: "mouseEventsHaveKeyEvents",
	enter(path: Path): TransformExitResult {
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

		return node;
	},
};
