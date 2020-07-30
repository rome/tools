import {descriptions} from "@internal/diagnostics";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isJSXElement,
} from "@internal/js-ast-utils";
import {createVisitor, signals} from "@internal/compiler";

export default createVisitor({
	name: "jsx-a11y/noOnChange",
	enter(path) {
		const {context, node} = path;

		if (!isJSXElement(node, "select") && !isJSXElement(node, "option")) {
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
			descriptions.LINT.JSX_A11Y_NO_ON_CHANGE,
		);

		return signals.retain;
	},
});
