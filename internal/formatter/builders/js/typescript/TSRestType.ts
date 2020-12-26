import {Builder, Token, concat} from "@internal/formatter";
import {TSRestType} from "@internal/ast";

export default function TSRestType(builder: Builder, node: TSRestType): Token {
	return concat(["...", builder.tokenize(node.argument, node)]);
}
