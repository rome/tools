import {AnyNode} from "@internal/ast";
import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

/**
 * Recurses through `node` to check if it's a regex of only single characters and alternations e.g. a|b|c
 */
function recurseCharAlternations(node: AnyNode): boolean {
	switch (node.type) {
		case "JSRegExpAlternation":
			return (
				recurseCharAlternations(node.left) &&
				recurseCharAlternations(node.right)
			);

		case "JSRegExpSubExpression":
			return node.body.length === 1 && recurseCharAlternations(node.body[0]);

		case "JSRegExpCharacter":
			return true;

		default:
			return false;
	}
}

function isCharAlternations(node: AnyNode): boolean {
	if (node.type !== "JSRegExpAlternation") {
		return false;
	}

	return recurseCharAlternations(node);
}

export default createVisitor({
	name: "js/noSingleCharRegexAlternatives",
	enter(path) {
		const {node} = path;

		if (isCharAlternations(node)) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JS_NO_SINGLE_CHAR_REGEX_ALTERNATIVES,
			);
		}

		return signals.retain;
	},
});
