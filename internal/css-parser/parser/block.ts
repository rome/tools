import {AnyCSSValue, CSSParser} from "@internal/css-parser/types";
import {CSSAtRule, CSSBlock, CSSDeclaration, CSSRule} from "@internal/ast";
import {
	getBlockEndTokenType,
	getBlockStartTokenValue,
} from "@internal/css-parser/utils";
import {matchToken, nextToken} from "@internal/css-parser/tokenizer";
import {parseRules} from "@internal/css-parser/parser/rules";
import {descriptions} from "@internal/diagnostics";
import {parseComponentValue} from "@internal/css-parser/parser/value";

export function parseComplexBlock(parser: CSSParser): CSSBlock | undefined {
	const start = parser.getPosition();
	const startingToken = parser.getToken();
	const startingTokenValue = getBlockStartTokenValue(parser, startingToken);
	const endingTokenType = getBlockEndTokenType(parser, startingToken);
	let value: Array<CSSAtRule | CSSRule> = [];

	if (!endingTokenType) {
		return undefined;
	}

	nextToken(parser);

	value = parseRules(parser, false, endingTokenType);

	return parser.finishNode(
		start,
		{
			type: "CSSBlock",
			startingTokenValue,
			value,
		},
	);
}

export function parseSimpleBlock(parser: CSSParser): CSSBlock | undefined {
	const start = parser.getPosition();
	const startingToken = parser.getToken();
	const startingTokenValue = getBlockStartTokenValue(parser, startingToken);
	const endingTokenType = getBlockEndTokenType(parser, startingToken);
	let value: Array<AnyCSSValue | CSSAtRule | CSSDeclaration> = [];

	if (!endingTokenType) {
		return undefined;
	}

	nextToken(parser);

	while (true) {
		if (matchToken(parser, endingTokenType)) {
			nextToken(parser);
			break;
		}
		if (matchToken(parser, "EOF")) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_BLOCK,
				token: parser.getToken(),
			});
			break;
		}
		const parsedValue = parseComponentValue(parser);
		parsedValue && value.push(parsedValue);
	}

	return parser.finishNode(
		start,
		{
			type: "CSSBlock",
			startingTokenValue,
			value,
		},
	);
}
