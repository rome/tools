import {CSSParser, Tokens} from "@internal/css-parser/types";
import {CSSSupportsCondition, CSSSupportsInParens} from "@internal/ast";
import {matchToken, nextToken, readToken} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";
import {AND, CONDITIONS, OR} from "@internal/css-parser/utils";
import {parseDeclaration} from "@internal/css-parser/parser/declaration";
import {Position} from "@internal/parser-core";

function parseAtSupportsInParens(
	parser: CSSParser,
	prefixStart?: Position,
	prefix?: string,
): CSSSupportsInParens | undefined {
	const token = parser.getToken();

	if (token.type === "LeftParen") {
		const start = prefixStart ?? parser.getPosition();
		parser.nextToken();
		while (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
		}

		const token = parser.getToken();
		if (token.type === "Ident" && CONDITIONS.includes(token.value)) {
			const condition = parseAtSupports(parser);
			if (condition) {
				return parser.finishNode(
					start,
					{
						type: "CSSSupportsInParens",
						value: condition,
						prefix,
					},
				);
			}
		} else {
			if (token.type === "LeftParen") {
				const declaration = parseAtSupports(parser);
				if (declaration) {
					return parser.finishNode(
						start,
						{
							type: "CSSSupportsInParens",
							value: declaration,
							prefix,
						},
					);
				}
				return undefined;
			}
			const declaration = parseDeclaration({
				parser,
				endingTokenType: "RightParen",
			});
			if (declaration) {
				const feature = parser.finishNode(
					start,
					{
						type: "CSSSupportsFeature",
						value: parser.finishNode(
							start,
							{
								type: "CSSSupportsDeclaration",
								value: declaration,
							},
						),
					},
				);
				if (feature) {
					return parser.finishNode(
						start,
						{
							type: "CSSSupportsInParens",
							value: feature,
							prefix,
						},
					);
				}
			}
		}
	}

	return undefined;
}

export function parseAtSupports(
	parser: CSSParser,
): CSSSupportsCondition | undefined {
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}

	if (!(parser.matchToken("Ident") || parser.matchToken("LeftParen"))) {
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.AT_SUPPORTS_MALFORMED,
			token: parser.getToken(),
		});
		nextToken(parser);
		return undefined;
	}

	const start = parser.getPosition();
	if (parser.matchToken("LeftParen")) {
		const inParens = parseAtSupportsInParens(parser);
		if (inParens) {
			const value = [inParens];
			while (true) {
				parser.nextToken();

				while (matchToken(parser, "Whitespace")) {
					readToken(parser, "Whitespace");
				}

				if (
					parser.matchToken("LeftCurlyBracket") ||
					parser.matchToken("RightParen")
				) {
					break;
				}

				const token = parser.getToken();
				if (
					token.type === "Ident" &&
					(token.value === AND || token.value === OR)
				) {
					const pos = parser.getPosition();
					parser.nextToken();
					while (matchToken(parser, "Whitespace")) {
						readToken(parser, "Whitespace");
					}
					const inParens = parseAtSupportsInParens(parser, pos, token.value);
					if (inParens) {
						value.push(inParens);
					}
				} else {
					parser.unexpectedDiagnostic({
						description: descriptions.CSS_PARSER.AT_SUPPORTS_MALFORMED,
						token,
					});
					parser.nextToken();
					break;
				}
			}
			return parser.finishNode(
				start,
				{
					type: "CSSSupportsCondition",
					value,
				},
			);
		}
	} else {
		const prefix = (parser.getToken() as Tokens["Ident"]).value;
		const position = parser.getPosition();
		parser.nextToken();

		while (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
		}

		const supportsInParens = parseAtSupportsInParens(parser, position, prefix);
		if (supportsInParens) {
			const value = [supportsInParens];
			parser.nextToken();
			while (matchToken(parser, "Whitespace")) {
				readToken(parser, "Whitespace");
			}
			return parser.finishNode(
				start,
				{
					type: "CSSSupportsCondition",
					value,
				},
			);
		}
	}

	return undefined;
}
