import {CSSParser} from "@internal/css-parser/types";
import {CSSGridAreaValue, CSSGridLine} from "@internal/ast";
import {
	matchToken,
	nextToken,
	skipWhitespaces,
} from "@internal/css-parser/tokenizer";
import {parseGridLine} from "@internal/css-parser/parser/grid/index";
import {descriptions} from "@internal/diagnostics";

// https://developer.mozilla.org/en-US/docs/Web/CSS/grid-area#formal_syntax
const MAX_NUMBER_OF_GRID_LINES = 4;

export function parseGridArea(parser: CSSParser): CSSGridAreaValue | undefined {
	const start = parser.getPosition();
	const value: CSSGridLine[] = [];

	const lines = parseGridLine(parser);
	if (!lines) {
		return undefined;
	}

	let currentGridLineIndex = 1;
	value.push(lines);

	while (true) {
		if (currentGridLineIndex > MAX_NUMBER_OF_GRID_LINES) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.GRID_AREA_TOO_MANY_GRID_LINES,
				start,
				end: parser.getPosition(),
			});
			return undefined;
		}
		skipWhitespaces(parser);
		const token = parser.getToken();
		if (token.type === "Semi") {
			break;
		}
		if (token.type === "Delim") {
			if (token.value !== "/") {
				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.GRID_AREA_INCORRECT_DELIMITER,
					token,
				});
				return undefined;
			}
			nextToken(parser);
			skipWhitespaces(parser);
			if (!(matchToken(parser, "Ident") || matchToken(parser, "Number"))) {
				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.GRID_AREA_UNTERMINATED_GRID_LINE,
					token: parser.getToken(),
				});
				return undefined;
			}
			const gridLine = parseGridLine(parser);
			if (gridLine) {
				value.push(gridLine);
				currentGridLineIndex += 1;
			} else {
				return undefined;
			}
		}
		// TODO something wrong, create diagnostic
		// return undefined;
	}

	return parser.finishNode(
		start,
		{
			type: "CSSGridAreaValue",
			value,
		},
	);
}
