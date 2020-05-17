import {Path, TransformExitResult} from "@romejs/js-compiler";
import {descriptions} from "@romejs/diagnostics";
import {isJSXElement} from "@romejs/js-ast-utils";

const DISTRACTING_TYPES = ["blink", "marquee"];

export default {
	name: "jsxA11yNoDistractingElements",

	enter(path: Path): TransformExitResult {
		const {node} = path;

		const distractingType = isJSXElement(node) && DISTRACTING_TYPES.find((name) =>
			isJSXElement(node, name)
		);

		if (distractingType) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_NO_DISTRACTING_ELEMENTS(distractingType),
			);
		}

		return node;
	},
};
