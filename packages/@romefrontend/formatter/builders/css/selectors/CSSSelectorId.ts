import {CSSSelectorId} from "@romefrontend/ast";
import {Builder, Token, concat} from "@romefrontend/formatter";

export default function CSSSelectorId(
	builder: Builder,
	node: CSSSelectorId,
): Token {
	return concat(["#", builder.tokenize(node.id, node)]);
}
