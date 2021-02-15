import {CSSVarFunction} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";
import {printCommaList} from "@internal/formatter/builders/css/utils";

export default function CSSVarFunction(
	builder: Builder,
	node: CSSVarFunction,
): Token {
	return concat([
		node.name,
		"(",
		printCommaList(builder, node.params, node),
		")",
	]);
}
