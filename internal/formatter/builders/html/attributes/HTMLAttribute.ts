import {HTMLAttribute} from "@romefrontend/ast";
import {Builder, Token, concat} from "@romefrontend/formatter";

export default function HTMLAttribute(
	builder: Builder,
	node: HTMLAttribute,
): Token {
	return concat([
		builder.tokenize(node.name, node),
		"=",
		builder.tokenize(node.value, node),
	]);
}
