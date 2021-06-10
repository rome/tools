// https://www.w3.org/TR/css-cascade-4/#conditional-import
import {CSSParser} from "@internal/css-parser/types";
import {CSSAtImport, CSSAtImportValue} from "@internal/ast";
import {matchToken, nextToken, readToken} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";

export function parseAtImport(parser: CSSParser): CSSAtImport | undefined {
	const start = parser.getPosition();
	let value: CSSAtImportValue;
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	const functionArgumentToken = parser.getToken();

	if (functionArgumentToken.type === "String") {
		nextToken(parser);
		value = functionArgumentToken.value;
		return parser.finishNode(
			start,
			{
				type: "CSSAtImport",
				value
			}
		);
	}

	const token = parser.getToken();

	if (token.type === "Function") {
		nextToken(parser);
		value = token.value;
		return parser.finishNode(
			start,
			{
				type: "CSSAtImport",
				value
			}
		);
	}

	if (token)
	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.AT_IMPORT_INVALID_ARGUMENT, // we need to create this diagnostic,
		token: functionArgumentToken
	});
	return undefined;
}
