import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {isJSXDOMElement} from "@internal/js-ast-utils/isJSXDOMElement";
import {doesNodeMatchPattern} from "@internal/js-ast-utils";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";

const DISTRACTING_TYPES = ["blink", "marquee"];

export default createLintVisitor({
	name: "a11y/noDistractingElements",
	enter(path) {
		const {node} = path;

		if (isHTMLElement(node)) {
			const distractingType = DISTRACTING_TYPES.find((name) =>
				name === node.name.name
			);
			if (distractingType) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.A11_Y_NO_DISTRACTING_ELEMENTS(distractingType),
				);
			}
		} else {
			const distractingType =
				isJSXDOMElement(node) &&
				DISTRACTING_TYPES.find((name) => doesNodeMatchPattern(node.name, name));

			if (distractingType) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.A11_Y_NO_DISTRACTING_ELEMENTS(distractingType),
				);
			}
		}

		return signals.retain;
	},
});
