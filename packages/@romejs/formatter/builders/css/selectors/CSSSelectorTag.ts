import {CSSSelectorTag} from "@romejs/ast";
import {Builder, Token} from "@romejs/formatter";

export default function CSSSelectorTag(
	builder: Builder,
	node: CSSSelectorTag,
): Token {
	return builder.tokenize(node.tagName, node);
}
