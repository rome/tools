import {CSSMediaAnd} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function CSSMediaAnd(builder: Builder, node: CSSMediaAnd): Token {
	return concat(["and", space, builder.tokenize(node.value, node)]);
}
