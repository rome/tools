import {CSSIdSelector} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSIdSelector(
	builder: Builder,
	node: CSSIdSelector,
): Token {
	return concat(["#", node.value]);
}
