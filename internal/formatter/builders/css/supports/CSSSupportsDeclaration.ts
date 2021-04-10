import {CSSSupportsDeclaration} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSSupportsDeclaration(
	builder: Builder,
	node: CSSSupportsDeclaration,
): Token {
	return concat(["(", builder.tokenize(node.value, node), ")"]);
}
