import {HTMLAttribute} from "@romejs/ast";
import {Builder, Token, concat} from "@romejs/formatter";

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
