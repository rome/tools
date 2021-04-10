import {CSSPageSelectorList} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function CSSPageSelectorList(
	builder: Builder,
	node: CSSPageSelectorList,
): Token {
	return concat(
		node.value.map((child, index) => {
			if (index === 0) {
				return concat([builder.tokenize(child, node), space]);
			}
			return concat([builder.tokenize(child, node), ",", space]);
		}),
	);
}
