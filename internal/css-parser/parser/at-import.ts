// https://www.w3.org/TR/css-cascade-4/#conditional-import
import {CSSParser} from "@internal/css-parser/types";
import {CSSAtImport, CSSAtImportValue} from "@internal/ast";
import {matchToken, readToken, nextToken} from "@internal/css-parser/tokenizer";

export function parseAtImport(
	parser: CSSParser
): CSSAtImport | undefined {
	const start = parser.getPosition();
	let value: CSSAtImportValue;
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	let token = parser.getToken();

	if (token.type === "String") {
		value = [token.value];
		return parser.finishNode(
			start,
			{
				type: "CSSAtImport",
				value,
			}
		);
	}

	if (token.type === "Function") {
		nextToken(parser);
		token = parser.getToken();
		value = [token.value];
		return parser.finishNode(
			start,
			{
				type: "CSSAtImport",
				value
			}
		);
	}

	return undefined;
}
