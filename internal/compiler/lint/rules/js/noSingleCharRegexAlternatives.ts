import {
	AnyNode,
	JSRegExpCharacter,
	jsRegExpCharSet,
	jsRegExpCharacter,
	jsRegExpSubExpression,
} from "@internal/ast";
import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

/**
 * Recurses through `node` to check if it's a regex of only single characters and alternations e.g. a|b|c
 */
function recurseCharAlternations(
	node: AnyNode,
	chars: JSRegExpCharacter[],
): boolean {
	switch (node.type) {
		case "JSRegExpAlternation":
			return (
				recurseCharAlternations(node.left, chars) &&
				recurseCharAlternations(node.right, chars)
			);

		case "JSRegExpSubExpression":
			return (
				node.body.length === 1 && recurseCharAlternations(node.body[0], chars)
			);

		case "JSRegExpCharacter": {
			chars.push(jsRegExpCharacter.create({value: node.value}));
			return true;
		}

		default:
			return false;
	}
}

function getCharAlternations(node: AnyNode): (JSRegExpCharacter[]) | undefined {
	if (node.type !== "JSRegExpAlternation") {
		return;
	}
	const chars = Array<JSRegExpCharacter>();
	return recurseCharAlternations(node, chars) ? chars : undefined;
}

export default createLintVisitor({
	name: "js/noSingleCharRegexAlternatives",
	enter(path) {
		const {node} = path;

		const chars = getCharAlternations(node);
		if (chars) {
			return path.addFixableDiagnostic(
				{
					fixed: signals.replace(
						jsRegExpSubExpression.create({
							body: [
								jsRegExpCharSet.create({
									body: chars,
								}),
							],
						}),
					),
				},
				descriptions.LINT.JS_NO_SINGLE_CHAR_REGEX_ALTERNATIVES,
			);
		}

		return signals.retain;
	},
});
