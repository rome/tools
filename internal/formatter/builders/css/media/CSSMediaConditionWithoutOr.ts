import {CSSMediaConditionWithoutOr} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function CSSMediaConditionWithoutOr(
	builder: Builder,
	node: CSSMediaConditionWithoutOr,
): Token {
	const value = node.value;
	if (Array.isArray(value)) {
		return concat(
			value.map((n, index) => {
				if (index + 1 < value.length) {
					return concat([builder.tokenize(n, node), space]);
				}
				return builder.tokenize(n, node);
			}),
		);
	} else {
		return builder.tokenize(value, node);
	}
}
