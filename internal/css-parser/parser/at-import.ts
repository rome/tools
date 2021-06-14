// https://www.w3.org/TR/css-cascade-4/#conditional-import
import {CSSParser} from "@internal/css-parser/types";
import {CSSAtImport, CSSAtImportValue} from "@internal/ast";
import {matchToken, nextToken, readToken} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";
import {parseFunction} from "@internal/css-parser/parser/function";

export function parseAtImport(parser: CSSParser): CSSAtImport | undefined {
	const start = parser.getPosition();
	let value: CSSAtImportValue | undefined = undefined;
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	const token = parser.getToken();
	let isFunctionToken = false;

	if (token.type === "String") {
		const functionStart = parser.getPosition();
		nextToken(parser);
		value = parser.finishNode(
			functionStart,
			{
				type: "CSSString",
				value: token.value,
			},
		);
	} else if (token.type === "Function") {
		const urlFunction = parseFunction(parser);
		if (urlFunction?.type === "CSSUrlFunction") {
			value = urlFunction;
		}

		isFunctionToken = true;
	}

	if (value) {
		if (parser.getToken().type === "RightParen") {
			nextToken(parser);
		}
		// Semi colon is optional, but if present we have to move forward in order to not
		// break the existing usage and make it as part of the current node
		if (parser.getToken().type === "Semi") {
			nextToken(parser);
		} else {
			if (isFunctionToken) {
				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.UNTERMINATED_FUNCTION,
					token,
				});
				return undefined;
			}
		}
		return parser.finishNode(
			start,
			{
				type: "CSSAtImport",
				value,
			},
		);
	}
	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.AT_IMPORT_INVALID_ARGUMENT,
		token,
	});
	return undefined;
}
