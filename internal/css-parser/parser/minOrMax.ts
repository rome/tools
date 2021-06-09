import {CSSParser, Tokens} from "@internal/css-parser/types";
import {CSSCalcSum, CSSMaxFunction, CSSMinFunction} from "@internal/ast";
import {matchToken, nextToken, readToken} from "@internal/css-parser/tokenizer";
import {parseCalcSum} from "@internal/css-parser/parser/calculations";
import {descriptions} from "@internal/diagnostics";

export function parseMinOrMaxFunction(
	parser: CSSParser,
	name: string,
): CSSMinFunction | CSSMaxFunction | undefined {
	const params: CSSCalcSum[] = [];
	// prepare variables needed for the final node
	const previousToken = parser.getPreviousToken() as Tokens["Ident"];
	const start = parser.getPositionFromIndex(previousToken.start);

	// skip possible white spaces
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}

	// push the first calculus
	const calcSum = parseCalcSum(parser);
	if (calcSum) {
		params.push(calcSum);
	}

	while (true) {
		// skip possible white spaces
		while (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
		}
		// found a comma, let's parse a new calc sum
		if (matchToken(parser, "Comma")) {
			nextToken(parser);
			// skip possible white spaces
			while (matchToken(parser, "Whitespace")) {
				readToken(parser, "Whitespace");
			}
			// parse the calc sum
			const calcSum = parseCalcSum(parser);
			if (calcSum) {
				params.push(calcSum);
			}
		} else if (matchToken(parser, "RightParen")) {
			nextToken(parser);
			break;
		} else {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_FUNCTION,
			});
			return undefined;
		}
	}

	return parser.finishNode(
		start,
		{
			type: name === "min" ? "CSSMinFunction" : "CSSMaxFunction",
			params,
			name,
		},
	);
}
