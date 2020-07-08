import {CSSSelectorClass} from "@romefrontend/ast";
import {Builder, Token, concat} from "@romefrontend/formatter";

export default function CSSSelectorClass(
	builder: Builder,
	node: CSSSelectorClass,
): Token {
	return concat([".", builder.tokenize(node.className, node)]);
}
