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
  return concat([
    "#".repeat(node.level),
    space,
    node.value,
    hardline,
  ]);
}
