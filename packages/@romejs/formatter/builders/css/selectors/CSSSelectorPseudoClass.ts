import {CSSSelectorPseudoClass} from "@romejs/ast";
import {Builder, Token, concat} from "@romejs/formatter";

export default function CSSSelectorPseudoClass(
	builder: Builder,
	node: CSSSelectorPseudoClass,
): Token {
	return concat([":", builder.tokenize(node.name, node)]);
}
