import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {hasJSXAttribute} from "@romejs/js-ast-utils";

export default {
	name: "clickEventsHaveKeyEvents",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (
			node.type === "JSXElement" &&
			hasJSXAttribute(node, "onClick") &&
			!(hasJSXAttribute(node, "onKeyUp") ||
			hasJSXAttribute(node, "onKeyDown") ||
			hasJSXAttribute(node, "onKeyPress"))
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_CLICK_EVENTS_HAVE_KEY_EVENTS,
			);
		}

		return node;
	},
};
