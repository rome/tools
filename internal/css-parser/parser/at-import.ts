// https://www.w3.org/TR/css-cascade-4/#conditional-import
import {CSSParser} from "@internal/css-parser/types";
import {CSSAtImport, CSSAtImportValue} from "@internal/ast";
import {matchToken, nextToken, readToken} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";

export function parseAtImport(parser: CSSParser): CSSAtImport | undefined {
	const start = parser.getPosition();
	let value: CSSAtImportValue | undefined = undefined;
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	const functionArgumentToken = parser.getToken();

	if (functionArgumentToken.type === "String") {
		nextToken(parser);
		value = functionArgumentToken.value;
	}

	const token = parser.getToken();

	if (token.type === "Function") {
		nextToken(parser);
		value = token.value;
	}

	if (value) {
		nextToken(parser)
		// Semi colon is optional, but if present we have to move forward in order to not
		// break the existing usage and make it as part of the current node
		if (parser.getToken().type === "Semi") {
			nextToken(parser);
		}
		return parser.finishNode(
			start,
			{
				type: "CSSAtImport",
				value
			}
		);
	}
	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.AT_IMPORT_INVALID_ARGUMENT, // we need to create this diagnostic,
		token: functionArgumentToken
	});
	return undefined;
}
