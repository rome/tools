import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {doesNodeMatchPattern} from "@internal/js-ast-utils";
import {isDomElement} from "@internal/js-ast-utils/isDomElement";

const DISTRACTING_TYPES = ["blink", "marquee"];

export default createVisitor({
	name: "jsx-a11y/noDistractingElements",

	enter(path) {
		const {node} = path;

		const distractingType =
			isDomElement(node) &&
			DISTRACTING_TYPES.find((name) => doesNodeMatchPattern(node.name, name));

		if (distractingType) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_NO_DISTRACTING_ELEMENTS(distractingType),
			);
		}

		return signals.retain;
	},
});
