import {CSSSelectorTag} from "@romefrontend/ast";
import {Builder, Token} from "@romefrontend/formatter";

export default function CSSSelectorTag(
	builder: Builder,
	node: CSSSelectorTag,
): Token {
	return builder.tokenize(node.tagName, node);
}
