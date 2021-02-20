import {CSSClassSelector} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSClassSelector(
	builder: Builder,
	node: CSSClassSelector,
): Token {
	return concat([".", node.value]);
}
