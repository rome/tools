import {CSSParser, Tokens} from "@internal/css-parser/types";
import {
	CSSAtRule,
	CSSBlock,
	CSSCustomProperty,
	CSSDeclaration,
} from "@internal/ast";
import {matchToken, nextToken, readToken} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";
import {
	getBlockEndTokenType,
	getBlockStartTokenValue,
	isCustomProperty,
	matchEndOfDeclaration,
} from "@internal/css-parser/utils";
import {parseAtRule} from "@internal/css-parser/parser/rules";
import {parseComponentValue} from "@internal/css-parser/parser/value";
import {AnyCSSValue} from "@internal/ast/css/unions";

export function parseDeclarations(
	parser: CSSParser,
	endingTokenType: keyof Tokens,
): Array<CSSAtRule | CSSDeclaration> {
	const declarations: Array<CSSAtRule | CSSDeclaration> = [];

	while (!matchToken(parser, "EOF")) {
		if (readToken(parser, "Whitespace") || readToken(parser, "Semi")) {
			continue;
		}
		if (matchToken(parser, endingTokenType)) {
			nextToken(parser);
			break;
		}
		if (matchToken(parser, "AtKeyword")) {
			declarations.push(parseAtRule(parser));
			continue;
		}
		if (matchToken(parser, "Ident")) {
			while (!matchEndOfDeclaration(parser, endingTokenType)) {
				const declaration = parseDeclaration(parser, endingTokenType);
				declaration && declarations.push(declaration);
			}
			continue;
		}
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.INVALID_DECLARATION,
			token: parser.getToken(),
		});
		while (!(matchToken(parser, "Semi") || matchToken(parser, "EOF"))) {
			parseComponentValue(parser);
		}
	}

	return declarations;
}

export function parseDeclaration(
	parser: CSSParser,
	endingTokenType: keyof Tokens,
): CSSDeclaration | undefined {
	while (!matchEndOfDeclaration(parser, endingTokenType)) {
		const currentToken = parser.getToken();
		if (currentToken.type !== "Ident") {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.INVALID_DECLARATION,
				token: parser.getToken(),
			});
			return undefined;
		}
		let name: string | CSSCustomProperty;
		if (isCustomProperty(currentToken.value)) {
			name = parser.finishNode(
				parser.getPosition(),
				{
					type: "CSSCustomProperty",
					value: currentToken.value,
				},
			);
		} else {
			name = currentToken.value;
		}
		const start = parser.getPosition();
		let important = false;
		let value: Array<AnyCSSValue | undefined> = [];
		nextToken(parser);

		while (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
		}
		if (!matchToken(parser, "Colon")) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.INVALID_DECLARATION,
				token: parser.getToken(),
			});
			return undefined;
		}
		nextToken(parser);
		while (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
		}
		while (!matchEndOfDeclaration(parser, endingTokenType)) {
			const parsedValue = parseComponentValue(parser);
			parsedValue && value.push(parsedValue);
		}

		if (value.length >= 2) {
			const lastTwoTokens = [...value].slice(-2);
			if (
				lastTwoTokens[0]?.type === "CSSRaw" &&
				lastTwoTokens[0].value === "!" &&
				lastTwoTokens[1]?.type === "CSSIdentifier" &&
				/^important$/i.test(lastTwoTokens[1].value)
			) {
				value = value.slice(0, -2);
				important = true;
			}
		}

		return parser.finishNode(
			start,
			{
				type: "CSSDeclaration",
				important,
				name,
				value,
			},
		);
	}
	return undefined;
}

export function parseDeclarationBlock(parser: CSSParser): CSSBlock | undefined {
	const start = parser.getPosition();
	const startingToken = parser.getToken();
	const startingTokenValue = getBlockStartTokenValue(parser, startingToken);
	const endingTokenType = getBlockEndTokenType(parser, startingToken);
	let value: Array<AnyCSSValue | CSSAtRule | CSSDeclaration> = [];

	if (!endingTokenType) {
		return undefined;
	}

	nextToken(parser);

	value = parseDeclarations(parser, endingTokenType);

	return parser.finishNode(
		start,
		{
			type: "CSSBlock",
			startingTokenValue,
			value,
		},
	);
}
