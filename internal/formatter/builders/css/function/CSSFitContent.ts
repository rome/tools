import {CSSFitContent} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function cssFitContent(
	builder: Builder,
	node: CSSFitContent,
): Token {
	return concat([node.name, "(", builder.tokenizer(node.value, node), ")"]);
}
