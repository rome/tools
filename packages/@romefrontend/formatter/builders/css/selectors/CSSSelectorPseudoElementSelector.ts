import {CSSSelectorPseudoElementSelector} from "@romefrontend/ast";
import {Builder, Token, concat} from "@romefrontend/formatter";

export default function CSSSelectorPseudoElementSelector(
	builder: Builder,
	node: CSSSelectorPseudoElementSelector,
): Token {
	return concat(["::", builder.tokenize(node.name, node)]);
}
