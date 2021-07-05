import {CSSParser, Tokens} from "@internal/css-parser/types";
import {
	CSSGridRepeatParams,
	CSSGridRepeatTracker,
	CSSGridRepeatValues,
	CSSRepeatFunction,
} from "@internal/ast";
import {
	matchToken,
	nextToken,
	skipWhitespaces,
} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";

function parseRepeatValues(parser: CSSParser): CSSGridRepeatValues | undefined {
	const start = parser.getPosition();
	const token = parser.getToken();
	while (!(matchToken(parser, "EOF") || matchToken(parser, "RightParen"))) {
		if (token.type === "Dimension") {
			nextToken(parser);
			return parser.finishNode(
				start,
				{
					type: "CSSDimension",
					value: token.value,
					unit: token.unit,
				},
			);
		} else if (token.type === "Percentage") {
			nextToken(parser);
			return parser.finishNode(
				start,
				{
					type: "CSSPercentage",
					value: token.value,
				},
			);
		} else if (token.type === "LeftSquareBracket") {
			nextToken(parser);
			skipWhitespaces(parser);
			const ident = parser.eatToken("Ident");
			if (!ident) {
				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.GRID_REPEAT_EXPECTED_IDENTIFIER,
					token: parser.getToken(),
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
					token: parser.getToken(),
				});
				return undefined;
			}

			nextToken(parser);
			return parser.finishNode(
				start,
				{
					type: "CSSLineName",
					value: ident.value,
				},
			);
		}
	}

	return undefined;
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
