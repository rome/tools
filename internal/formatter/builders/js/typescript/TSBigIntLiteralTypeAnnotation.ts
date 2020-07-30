import {TSBigIntLiteralTypeAnnotation} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function TSBigIntLiteralTypeAnnotation(
	builder: Builder,
	node: TSBigIntLiteralTypeAnnotation,
): Token {
	return `${node.value}n`;
}
