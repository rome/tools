import {CSSParser, Tokens} from "@internal/css-parser/types";
import {
	CSSDimension,
	CSSFitContentFunction,
	CSSNumber,
	CSSPercentage,
} from "@internal/ast";
import {
	matchToken,
	nextToken,
	skipWhitespaces,
} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";

export function parseFitContentFunction(
	parser: CSSParser,
): CSSFitContentFunction | undefined {
	// prepare variables needed for the final node
	const previousToken = parser.getPreviousToken() as Tokens["Ident"];
	const functionStart = parser.getPositionFromIndex(previousToken.start);
	const start = parser.getPosition();

	skipWhitespaces(parser);

	if (
		matchToken(parser, "Number") ||
		matchToken(parser, "Percentage") ||
		matchToken(parser, "Dimension")
	) {
		const token = parser.getToken();
		let value: CSSNumber | CSSPercentage | CSSDimension;
		if (token.type === "Number") {
			nextToken(parser);
			value = parser.finishNode(
				start,
				{
					type: "CSSNumber",
					raw: token.raw,
					value: token.value,
				},
			);
		} else if (token.type === "Dimension") {
			nextToken(parser);
			value = parser.finishNode(
				start,
				{
					type: "CSSDimension",
					value: token.value,
					unit: token.unit,
				},
			);
		} else {
			nextToken(parser);
			value = parser.finishNode(
				start,
				{
					type: "CSSPercentage",
					value: (token as Tokens["Percentage"]).value,
				},
			);
		}
		skipWhitespaces(parser);
		if (matchToken(parser, "RightParen")) {
			nextToken(parser);
			return parser.finishNode(
				functionStart,
				{
					type: "CSSFitContentFunction",
					name: "fit-content",
					params: [value],
				},
			);
		}
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.UNTERMINATED_FUNCTION,
			token: parser.getToken(),
		});
		return undefined;
	}

	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.FIT_CONTENT_UNKOWN_FUNCTION,
		token: parser.getToken(),
	});

	return undefined;
}
