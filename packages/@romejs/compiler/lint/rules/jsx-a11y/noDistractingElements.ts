import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {doesNodeMatchPattern} from "@romejs/js-ast-utils";

const DISTRACTING_TYPES = ["blink", "marquee"];

export default {
	name: "jsxA11YNoDistractingElements",

	enter(path: Path): TransformExitResult {
		const {node} = path;

		const distractingType =
			node.type === "JSXElement" &&
			DISTRACTING_TYPES.find((name) => doesNodeMatchPattern(node.name, name));

		if (distractingType) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_NO_DISTRACTING_ELEMENTS(distractingType),
			);
		}

		return node;
	},
};
