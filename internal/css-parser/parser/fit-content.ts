import {CSSParser, Tokens} from "@internal/css-parser/types";
import {
	CSSDimension,
	CSSFitContent,
	CSSNumber,
	CSSPercentage,
} from "@internal/ast";
import {matchToken, nextToken} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";

function parseFitContent(parser: CSSParser): CSSFitContent | undefined {
	// prepare variables needed for the final node
	const previousToken = parser.getPreviousToken() as Tokens["Ident"];
	const functionStart = parser.getPositionFromIndex(previousToken.start);
	const start = parser.getPosition();
	
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
		nextToken(parser);
		return parser.finishNode(
			functionStart,
			{
				type: "CSSFitContent",
				name: "fit-content",
				params: [value],
			},
		);
	}

	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.FIT_CONTENT_UNKOWN_FUNCTION,
		token: parser.getToken(),
	});

	return undefined;
}

export function parseFitContentFunction(
	parser: CSSParser,
): CSSFitContent | undefined {
	const value = parseFitContent(parser);

	if (value) {
		nextToken(parser);
		return value;
	}

	return undefined;
}
