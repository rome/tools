import {CSSParser} from "@internal/css-parser/types";
import {CSSString, CSSUrlFunction} from "@internal/ast";
import {matchToken, nextToken, readToken} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";

export function parseUrl(parser: CSSParser): CSSUrlFunction | undefined {
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}

	const start = parser.getPosition();
	let value: CSSString;
	if (matchToken(parser, "URL") || matchToken(parser, "BadURL")) {
		const token = parser.getToken();
		if (token.type === "URL") {
			const maybeNumber = Number(token.value);
			if (isNaN(maybeNumber)) {
				value = parser.finishNode(
					start,
					{
						type: "CSSString",
						value: token.value,
					},
				);
				nextToken(parser);
				return parser.finishNode(
					start,
					{
						type: "CSSUrlFunction",
						name: "url",
						params: [value],
					},
				);
			} else {
				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.URL_FUNCTION_INVALID_VALUE,
					token,
				});
				nextToken(parser);
				return undefined;
			}
		}
	}

	nextToken(parser);

	return undefined;
}
