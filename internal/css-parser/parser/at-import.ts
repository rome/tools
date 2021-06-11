// https://www.w3.org/TR/css-cascade-4/#conditional-import
import {CSSParser, Tokens} from "@internal/css-parser/types";
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
	let isFunctionToken = false;

	if (functionArgumentToken.type === "String") {
		const functionStart = parser.getPosition();
		nextToken(parser);
		value = parser.finishNode(functionStart, {
			type: "CSSString",
			value: functionArgumentToken.value,
		});
	}

	const token = parser.getToken();

	if (token.type === "Function") {
		const functionStart = parser.getPosition();
		nextToken(parser);
		const functionToken = parser.getToken() as Tokens["String"];
		value = parser.finishNode(functionStart, {
			type: "CSSUrlFunction",
			name: "url",
			params: [parser.finishNode(functionStart, {
				type: "CSSString",
				value: functionToken.value
			})]
		});
		isFunctionToken = true;
	}

	if (value) {
		nextToken(parser);
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
					token: token,
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
		token: functionArgumentToken,
	});
	return undefined;
}
