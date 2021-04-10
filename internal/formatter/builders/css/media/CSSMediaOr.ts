import {CSSMediaOr} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function CSSMediaOr(builder: Builder, node: CSSMediaOr): Token {
	return concat(["or", space, builder.tokenize(node.value, node)]);
}
