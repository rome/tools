import {CSSSelectorId} from "@romejs/ast";
import {Builder, Token, concat} from "@romejs/formatter";

export default function CSSSelectorId(
	builder: Builder,
	node: CSSSelectorId,
): Token {
	return concat(["#", builder.tokenize(node.id, node)]);
}
