import {CSSFunction} from "@internal/ast";
import {AnyCSSValue} from "@internal/css-parser/types";
import {Builder, Token, concat, join, space} from "@internal/formatter";

export default function CSSFunction(builder: Builder, node: CSSFunction): Token {
	const tokens: Token[] = [node.name, "("];

	const params: AnyCSSValue[] = [...node.params];
	const commaSeperatedTokens: Token[] = [];

	while (params.length) {
		const commaIndex = params.findIndex((param) => param?.type === "CSSComma");
		const values = params.splice(0, commaIndex > 0 ? commaIndex : params.length);
		commaSeperatedTokens.push(
			join(space, values.map((value) => builder.tokenize(value, node))),
		);
		params.splice(0, 1);
	}

	tokens.push(join(concat([",", space]), commaSeperatedTokens));
	tokens.push(")");

	return concat(tokens);
}
