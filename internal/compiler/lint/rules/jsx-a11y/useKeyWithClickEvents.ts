import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {hasJSXAttribute} from "@internal/js-ast-utils";

export default createVisitor({
	name: "jsx-a11y/useKeyWithClickEvents",
	enter(path) {
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

		return signals.retain;
	},
});
