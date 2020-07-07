import {TSBigIntLiteralTypeAnnotation} from "@romejs/ast";
import {Builder, Token} from "@romejs/formatter";

export default function TSBigIntLiteralTypeAnnotation(
	builder: Builder,
	node: TSBigIntLiteralTypeAnnotation,
): Token {
	return `${node.value}n`;
}
