import {Path, TransformExitResult} from "@romejs/js-compiler";
import {descriptions} from "@romejs/diagnostics";
import {hasJSXAttribute, isJSXElement} from "@romejs/js-ast-utils";

export default {
	name: "mouseEventsHaveKeyEvents",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (isJSXElement(node)) {
			if (
				hasJSXAttribute(node, "onMouseOver") &&
				!hasJSXAttribute(node, "onFocus")
			) {
				path.context.addNodeDiagnostic(
					node,
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
					node,
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
