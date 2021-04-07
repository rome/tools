import {CSSPseudoPage} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function CSSPseudoPage(
	builder: Builder,
	node: CSSPseudoPage,
): Token {
	return node.value;
}
