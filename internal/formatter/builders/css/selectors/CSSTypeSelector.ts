import {CSSTypeSelector} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function CSSTypeSelector(
	builder: Builder,
	node: CSSTypeSelector,
): Token {
	return node.value;
}
