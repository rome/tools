import {TSBigIntLiteralTypeAnnotation} from "@romefrontend/ast";
import {Builder, Token} from "@romefrontend/formatter";

export default function TSBigIntLiteralTypeAnnotation(
	builder: Builder,
	node: TSBigIntLiteralTypeAnnotation,
): Token {
	return `${node.value}n`;
}
