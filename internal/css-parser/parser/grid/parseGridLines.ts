import {CSSParser, Tokens} from "@internal/css-parser/types";
import {CSSNumber, CSSRaw} from "@internal/ast";
import {
	matchToken,
	nextToken,
	skipWhitespaces,
} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";

// https://developer.mozilla.org/en-US/docs/Web/CSS/grid-area#formal_syntax
const MAX_NUMBER_OF_GRID_LINES = 4;

function parseSpan(parser: CSSParser): Array<CSSRaw | CSSNumber> | undefined {
	const spanPosition = parser.getPosition();
	const span = parser.getToken() as Tokens["Ident"];
	const values: Array<CSSRaw | CSSNumber> = [
		parser.finishNode(
			spanPosition,
			{
				type: "CSSRaw",
				value: span.value,
			},
		),
	];
	nextToken(parser);
	skipWhitespaces(parser);
	const maybeNumberOrIdent = parser.getToken();
	if (
		maybeNumberOrIdent.type !== "Number" &&
		maybeNumberOrIdent.type !== "Ident"
	) {
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.GRID_AREA_INCORRECT_SPAN,
			token: maybeNumberOrIdent,
		});
		nextToken(parser);
		return undefined;
	}
	skipWhitespaces(parser);
	const secondPosition = parser.getPosition();
	if (maybeNumberOrIdent.type === "Number") {
		nextToken(parser);
		values.push(
			parser.finishNode(
				secondPosition,
				{
					type: "CSSNumber",
					value: maybeNumberOrIdent.value,
					raw: maybeNumberOrIdent.raw,
				},
			),
		);
	} else {
		nextToken(parser);
		values.push(
			parser.finishNode(
				secondPosition,
				{
					type: "CSSRaw",
					value: maybeNumberOrIdent.value,
				},
			),
		);
	}
	return values;
}

export function parseGridLine(
	parser: CSSParser,
): Array<CSSRaw | CSSNumber> | undefined {
	const token = parser.getToken();
	if (token.type === "Ident") {
		if (token.value === "span") {
			const spanValues = parseSpan(parser);
			if (spanValues) {
				return spanValues;
			}
			return undefined;
		}
		if (
			token.value === "auto" ||
			token.value === "inherit" ||
			token.value === "revert" ||
			token.value === "unset"
		) {
			const start = parser.getPosition();
			nextToken(parser);
			return [
				parser.finishNode(
					start,
					{
						type: "CSSRaw",
						value: token.value,
					},
				),
			];
		}
	}

	return undefined;
}

export function parseGridLines(
	parser: CSSParser,
	numberOfGridLines = MAX_NUMBER_OF_GRID_LINES,
): Array<CSSRaw | CSSNumber> | undefined {
	const start = parser.getPosition();
	const value: Array<CSSRaw | CSSNumber> = [];
	let hasError = false;
	const lines = parseGridLine(parser);
	if (!lines) {
		return undefined;
	}
	value.push(...lines);

	let currentGridLineIndex = 1;
	while (!matchToken(parser, "EOF")) {
		if (currentGridLineIndex > numberOfGridLines) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.GRID_AREA_TOO_MANY_GRID_LINES(
					numberOfGridLines,
				),
				start,
				end: parser.getPosition(),
			});
			nextToken(parser);
			hasError = true;
			break;
		}
		skipWhitespaces(parser);
		const token = parser.getToken();
		const pos = parser.getPosition();
		if (token.type === "Semi") {
			break;
		}
		if (token.type === "Delim") {
			if (token.value !== "/") {
				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.GRID_AREA_INCORRECT_DELIMITER,
					token,
				});
				nextToken(parser);
				hasError = true;
				break;
			}
			nextToken(parser);
			value.push(
				parser.finishNode(
					pos,
					{
						type: "CSSRaw",
						value: token.value,
					},
				),
			);
			skipWhitespaces(parser);
			if (!(matchToken(parser, "Ident") || matchToken(parser, "Number"))) {
				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.GRID_AREA_UNTERMINATED_GRID_LINE,
					token: parser.getToken(),
				});
				nextToken(parser);
				hasError = true;
				break;
			}
			const gridLine = parseGridLine(parser);
			if (gridLine) {
				value.push(...gridLine);
				currentGridLineIndex += 1;
			} else {
				hasError = true;
				break;
			}
		}
	}

	// dirty trick to finish the parsing early because there's an error
	// The reason why we use this gotcha is because this function is used inside
	if (hasError) {
		// while (!(matchToken(parser, "Semi") || matchToken(parser, "EOF"))) {
		// 	nextToken(parser);
		// }
		return undefined;
	}

	return value;
}
