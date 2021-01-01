import {CSSPseudoClassSelector} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSPseudoClassSelector(
	builder: Builder,
	node: CSSPseudoClassSelector,
): Token {
	return concat([":", node.value]);
}
