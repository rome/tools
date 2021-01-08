import {CSSPseudoElementSelector} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSPseudoElementSelector(
	builder: Builder,
	node: CSSPseudoElementSelector,
): Token {
	return concat(["::", node.value]);
}
