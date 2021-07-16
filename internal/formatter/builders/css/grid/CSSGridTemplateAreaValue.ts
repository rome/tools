import {CSSGridTemplateAreaValue} from "@internal/ast";
import {Builder, Token, join, space} from "@internal/formatter";

export default function CSSGridTemplateAreaValue(
	builder: Builder,
	node: CSSGridTemplateAreaValue,
): Token {
	return join(space, node.value.map((child) => builder.tokenize(child, node)));
}
