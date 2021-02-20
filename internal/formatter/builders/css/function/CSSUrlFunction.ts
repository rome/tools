import {CSSUrlFunction} from "@internal/ast";
import {Builder, Token, concat, group} from "@internal/formatter";

export default function CSSUrlFunction(
	builder: Builder,
	node: CSSUrlFunction,
): Token {
	return concat([
		node.name,
		group(concat(["(", builder.tokenize(node.params[0], node), ")"])),
	]);
}
