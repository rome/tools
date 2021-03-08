import {CSSMediaConditionWithoutOr} from "@internal/ast";
import {Builder, concat, space, Token} from "@internal/formatter";

export default function CSSMediaConditionWithoutOr(
	builder: Builder,
	node: CSSMediaConditionWithoutOr,
): Token {
	return concat([
		"and",
		space,
		builder.tokenize(node.value, node)
	])
}
