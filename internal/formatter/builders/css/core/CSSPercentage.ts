import {CSSPercentage} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSPercentage(
	builder: Builder,
	node: CSSPercentage,
): Token {
	return concat([String(node.value), "px"]);
}
