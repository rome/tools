import {CSSCombinator} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

const COMBINATOR_PRINT_MAP = {
	descendant: " ",
	child: ">",
	nextSibiling: "+",
	subsequentSibiling: "~",
} as const;

export default function CSSCombinator(
	builder: Builder,
	node: CSSCombinator,
): Token {
	const char = COMBINATOR_PRINT_MAP[node.combinator];

	if (node.combinator === "descendant") {
		return char;
	}

	return concat([space, char, space]);
}
