import {CSSParser, Tokens} from "@internal/css-parser/types";
import {
	CSSGridRepeatParams,
	CSSGridRepeatTracker,
	CSSGridRepeatValue,
	CSSGridRepeatValues,
	CSSRepeatFunction,
} from "@internal/ast";
import {
	matchToken,
	nextToken,
	skipWhitespaces,
} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";
import {parseMinmaxFunction} from "@internal/css-parser/parser/grid/minmax";
import {parseFitContentFunction} from "@internal/css-parser/parser/fit-content";

function parseRepeatValues(parser: CSSParser): CSSGridRepeatValue | undefined {
	const mainStart = parser.getPosition();
	const values: CSSGridRepeatValues = [];

	while (!(matchToken(parser, "EOF") || matchToken(parser, "RightParen"))) {
		const start = parser.getPosition();
		const token = parser.getToken();
		switch (token.type) {
			case "Dimension": {
				nextToken(parser);
				values.push(
					parser.finishNode(
						start,
						{
							type: "CSSDimension",
							value: token.value,
							unit: token.unit,
						},
					),
				);
				break;
			}

			case "Percentage": {
				nextToken(parser);
				values.push(
					parser.finishNode(
						start,
						{
							type: "CSSPercentage",
							value: token.value,
						},
					),
				);
				break;
			}

			case "LeftSquareBracket": {
				nextToken(parser);
				skipWhitespaces(parser);
				const ident = parser.eatToken("Ident");
				if (!ident) {
					parser.unexpectedDiagnostic({
						description: descriptions.CSS_PARSER.GRID_REPEAT_EXPECTED_IDENTIFIER,
						token,
					});
					return undefined;
				}
				skipWhitespaces(parser);
				const squareBracket = parser.eatToken("RightSquareBracket");
				if (!squareBracket) {
					parser.unexpectedDiagnostic({
						description: descriptions.CSS_PARSER.GRID_REPEAT_UNCLOSED_LINE_NAME(
							ident.value,
						),
						token,
					});
					return undefined;
				}

				values.push(
					parser.finishNode(
						start,
						{
							type: "CSSLineName",
							value: ident.value,
						},
					),
				);
				break;
			}

			case "Function": {
				const functionName = token.value;
				if (functionName === "minmax") {
					nextToken(parser);
					const minmaxFunction = parseMinmaxFunction(parser);
					if (minmaxFunction) {
						values.push(minmaxFunction);
						break;
					}
					return undefined;
				}
				if (functionName === "fit-content") {
					nextToken(parser);
					const fitContentFunction = parseFitContentFunction(parser);

					if (fitContentFunction) {
						values.push(fitContentFunction);
						break;
					}
					return undefined;
				}

				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.GRID_REPEAT_WRONG_FUNCTION,
					token,
				});
				return undefined;
			}

			case "Ident": {
				if (token.value === "min-content" || token.value === "max-content") {
					nextToken(parser);
					values.push(
						parser.finishNode(
							start,
							{
								type: "CSSRaw",
								value: token.value,
							},
						),
					);
					break;
				}
				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.GRID_REPEAT_WRONG_IDENTIFIER,
					token,
				});
				return undefined;
			}
		}

		if (matchToken(parser, "Whitespace")) {
			skipWhitespaces(parser);
			continue;
		}
		if (matchToken(parser, "RightParen")) {
			return parser.finishNode(
				mainStart,
				{
					type: "CSSGridRepeatValue",
					values,
				},
			);
		} else {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_FUNCTION,
				token: parser.getToken(),
			});
			nextToken(parser);
			return undefined;
		}
	}

	return parser.finishNode(
		mainStart,
		{
			type: "CSSGridRepeatValue",
			values,
		},
	);
}

function parseParams(parser: CSSParser): CSSGridRepeatParams | undefined {
	const start = parser.getPosition();
	const token = parser.getToken();
	let tracker: CSSGridRepeatTracker | undefined = undefined;
	if (token.type === "Number") {
		if (token.value < 1) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.GRID_REPEAT_POSITIVE_INTEGER,
				token,
			});
			return undefined;
		}
		nextToken(parser);
		tracker = parser.finishNode(
			start,
			{
				type: "CSSNumber",
				value: token.value,
				raw: token.raw,
			},
		);
	} else if (token.type === "Ident") {
		if (token.value !== "auto-fit" && token.value !== "auto-fill") {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.GRID_REPEAT_INCORRECT_IDENT,
				token,
			});
			return undefined;
		}
		nextToken(parser);
		tracker = parser.finishNode(
			start,
			{
				type: "CSSRaw",
				value: token.value,
			},
		);
	}
	skipWhitespaces(parser);
	const comma = parser.eatToken("Comma");
	if (!comma) {
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.GRID_REPEAT_TRACK_REPEAT_MANDATORY,
			token,
		});
		return undefined;
	}

	skipWhitespaces(parser);
	if (!matchToken(parser, "RightParen")) {
		const values = parseRepeatValues(parser);
		if (values && tracker) {
			return [tracker, values];
		}
	}

	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.GRID_REPEAT_UNTERMINATED,
		token: parser.getToken(),
	});
	return undefined;
}

export function parseRepeatFunction(
	parser: CSSParser,
): CSSRepeatFunction | undefined {
	// prepare variables needed for the final node
	const previousToken = parser.getPreviousToken() as Tokens["Ident"];
	const start = parser.getPositionFromIndex(previousToken.start);
	// starting by removing possible white spaces
	skipWhitespaces(parser);

	const params = parseParams(parser);

	if (params) {
		nextToken(parser);
		return parser.finishNode(
			start,
			{
				type: "CSSRepeatFunction",
				name: "repeat",
				params,
			},
		);
	}

	return undefined;
}
