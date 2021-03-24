import {CSSMediaNot} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function CSSMediaNot(builder: Builder, node: CSSMediaNot): Token {
	return concat(["not", space, builder.tokenize(node.value, node)]);
}
