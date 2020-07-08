import {CSSSelectorPseudoClass} from "@romefrontend/ast";
import {Builder, Token, concat} from "@romefrontend/formatter";

export default function CSSSelectorPseudoClass(
	builder: Builder,
	node: CSSSelectorPseudoClass,
): Token {
	return concat([":", builder.tokenize(node.name, node)]);
}
