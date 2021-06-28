import {CSSParser} from "@internal/css-parser/types";
import {CSSFitContent} from "@internal/ast";
import {matchToken, nextToken, readToken} from "@internal/css-parser/tokenizer";

function parseFitContent(parser: CSSParser): CSSFitContent | undefined {
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
		return parser.finishNode(
			start,
			{
				type: "CSSFitContent",
				value,
			},
		);
	} else if (matchToken(parser, "LeftParen")) {
		const result = parseCalcSum(parser);

		if (parser.getToken().type !== "RightParen") {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.FIT_CONTENT_UNTERMITED_FUNCTION,
				token: parser.getToken(),
			});
			nextToken(parser);
			return undefined;
		}
		if (result) {
			return parser.finishNode(
				start,
				{
					type: "CSSFitContent",
					value: result,
				},
			);
		}
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
	const start = parser.getPosition();
	const value = parseFitContent(parser);

	if (value) {
		nextToken(parser);
		return parser.finishNode(
			start,
			{
				type: "CSSFitContent",
				name: "fit-content",
				params: [value],
			},
		);
	}

	return undefined;
}
