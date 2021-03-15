import {CSSMediaCondition} from "@internal/ast";
import {Builder, concat, space, Token} from "@internal/formatter";

export default function CSSMediaCondition(
	builder: Builder,
	node: CSSMediaCondition,
): Token {
	const value = node.value
	if (Array.isArray(value)) {
		return concat(value.map(child => {
			return concat([builder.tokenize(child, node), space]);
		}))
	} else {
		return builder.tokenize(value, node);
	}
}
