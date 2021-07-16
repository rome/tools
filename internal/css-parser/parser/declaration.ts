import {CSSParser, Tokens} from "@internal/css-parser/types";
import {
	AnyCSSValue,
	CSSAtRule,
	CSSBlock,
	CSSCustomProperty,
	CSSDeclaration,
} from "@internal/ast";
import {
	matchToken,
	nextToken,
	readToken,
	skipWhitespaces,
} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";
import {
	getBlockEndTokenType,
	getBlockStartTokenValue,
	isCustomProperty,
	matchEndOfDeclaration,
} from "@internal/css-parser/utils";
import {parseAtRule} from "@internal/css-parser/parser/rules";
import {parseComponentValue} from "@internal/css-parser/parser/value";
import {parseGridLines} from "@internal/css-parser/parser/grid/parseGridLines";
import {parseTemplateAreas} from "@internal/css-parser/parser/grid/parseTemplateAreas";

interface ParseDeclarations {
	parser: CSSParser;
	endingTokenType: keyof Tokens;
	onAtKeyword?: OnAtKeyword;
	onAtDeclaration?: OnAtDeclaration;
	parentAtKeywordToken?: Tokens["AtKeyword"];
}

export function parseDeclarations(
	{
		parser,
		endingTokenType,
		onAtKeyword,
		onAtDeclaration,
		parentAtKeywordToken,
	}: ParseDeclarations,
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
			const token = parser.getToken() as Tokens["AtKeyword"];
			if (onAtKeyword) {
				const allowed = onAtKeyword(token);
				if (!allowed) {
					nextToken(parser);
					continue;
				}
			}
			declarations.push(parseAtRule({parser, onAtDeclaration, onAtKeyword}));
			continue;
		}
		if (matchToken(parser, "Ident")) {
			while (!matchEndOfDeclaration(parser, endingTokenType)) {
				const declaration = parseDeclaration({
					parser,
					endingTokenType,
					onAtDeclaration,
					parentAtKeywordToken,
				});
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

interface ParseDeclaration {
	parser: CSSParser;
	endingTokenType: keyof Tokens;
	onAtDeclaration?: OnAtDeclaration;
	parentAtKeywordToken?: Tokens["AtKeyword"];
}

export function parseDeclaration(
	{onAtDeclaration, parentAtKeywordToken, endingTokenType, parser}: ParseDeclaration,
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

		onAtDeclaration?.(currentToken, parentAtKeywordToken);

		let name: string | CSSCustomProperty;
		const start = parser.getPosition();
		if (isCustomProperty(currentToken.value)) {
			const pos = parser.getPosition();
			nextToken(parser);
			name = parser.finishNode(
				pos,
				{
					type: "CSSCustomProperty",
					value: currentToken.value,
				},
			);
		} else {
			name = currentToken.value;
			nextToken(parser);
		}
		let important = false;
		let value: Array<AnyCSSValue | undefined> = [];

		skipWhitespaces(parser);
		if (!matchToken(parser, "Colon")) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.INVALID_DECLARATION,
				token: parser.getToken(),
			});
			return undefined;
		}
		nextToken(parser);
		skipWhitespaces(parser);
		if (name && typeof name === "string") {
			/**
			 * If there's an error, we break from the switch and we continue the parsing.
			 *
			 * The error is carried over by the parser so the parsing won't break in any case.
			 */
			switch (name) {
				case "grid-column-end":
				case "grid-column-start":
				case "grid-row-end":
				case "grid-row-start": {
					const gridLine = parseGridLines(parser, 1);
					skipWhitespaces(parser);
					if (gridLine) {
						if (!matchToken(parser, "Semi")) {
							nextToken(parser);
							parser.unexpectedDiagnostic({
								description: descriptions.CSS_PARSER.GRID_AREA_UNTERMINATED_GRID_LINE,
								token: parser.getToken(),
							});
							// the previous function doesn't
							return undefined;
						}
						value.push(...gridLine);
					}
					// there's been an error, so we exist from the function
					break;
				}
				case "grid-area":
				case "grid-column":
				case "grid-row": {
					const gridArea = parseGridLines(parser);
					if (gridArea) {
						value.push(...gridArea);
					}
					// there's been an error, so we exist from the function
					break;
				}

				case "grid-template-areas": {
					const templateValue = parseTemplateAreas(parser);
					if (templateValue) {
						value.push(templateValue);
					}
					// there's been an error, so we exist from the function
					break;
				}
				default:
					break;
			}
		}
		while (!matchEndOfDeclaration(parser, endingTokenType)) {
			const parsedValue = parseComponentValue(parser, name);
			if (parsedValue) {
				if (Array.isArray(parsedValue)) {
					value.push(...parsedValue);
				} else {
					value.push(parsedValue);
				}
			}
		}

		// matching "important" inside a declaration
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

export type OnAtKeyword = (token: Tokens["AtKeyword"]) => boolean;
export type OnAtDeclaration = (
	token: Tokens["Ident"],
	previousAtKeywordToken?: Tokens["AtKeyword"],
) => boolean;

interface ParseDeclarationBlock {
	parser: CSSParser;
	onAtKeyword?: OnAtKeyword;
	onAtDeclaration?: OnAtDeclaration;
	parentAtKeywordToken?: Tokens["AtKeyword"];
}

export function parseDeclarationBlock(
	{parser, onAtKeyword, onAtDeclaration, parentAtKeywordToken}: ParseDeclarationBlock,
): CSSBlock | undefined {
	const start = parser.getPosition();
	const startingToken = parser.getToken();
	const startingTokenValue = getBlockStartTokenValue(parser, startingToken);
	const endingTokenType = getBlockEndTokenType(parser, startingToken);
	let value: Array<AnyCSSValue | CSSAtRule | CSSDeclaration> = [];

	if (!endingTokenType) {
		return undefined;
	}

	nextToken(parser);

	value = parseDeclarations({
		parser,
		endingTokenType,
		onAtKeyword,
		onAtDeclaration,
		parentAtKeywordToken,
	});

	return parser.finishNode(
		start,
		{
			type: "CSSBlock",
			startingTokenValue,
			value,
		},
	);
}
