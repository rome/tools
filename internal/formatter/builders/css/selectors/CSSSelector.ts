import {CSSSelector} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSSelector(builder: Builder, node: CSSSelector): Token {
	return concat(node.patterns.map((pattern) => builder.tokenize(pattern, node)));
}
