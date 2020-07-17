import {MarkdownHeadingBlock} from "@romefrontend/ast";
import {
	Builder,
	Token,
	Tokens,
	concat,
	hardline,
	space,
} from "@romefrontend/formatter";

export default function MarkdownHeadingBlock(
	builder: Builder,
	node: MarkdownHeadingBlock,
): Token {
	const tokens: Tokens = [...Array.from({length: node.level}).map(() => "#")];

	tokens.push(space);

	tokens.push(node.value);

	return concat([...tokens, hardline]);
}
