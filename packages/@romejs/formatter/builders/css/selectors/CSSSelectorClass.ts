import {CSSSelectorClass} from "@romejs/ast";
import {Builder, Token, concat} from "@romejs/formatter";

export default function CSSSelectorClass(
	builder: Builder,
	node: CSSSelectorClass,
): Token {
	return concat([".", builder.tokenize(node.className, node)]);
}
