import {CSSSupportsCondition} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSSupportsCondition(
	builder: Builder,
	node: CSSSupportsCondition,
): Token {
	return concat(
		node.value.map((child) => {
			return concat([builder.tokenize(child, node)]);
		}),
	);
}
