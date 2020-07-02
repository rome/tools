import {CSSSelectorPseudoElementSelector} from "@romejs/ast";
import {Builder, Token, concat} from "@romejs/formatter";

export default function CSSSelectorPseudoElementSelector(
	builder: Builder,
	node: CSSSelectorPseudoElementSelector,
): Token {
	return concat(["::", builder.tokenize(node.name, node)]);
}
