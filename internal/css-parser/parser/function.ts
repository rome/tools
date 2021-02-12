import {AnyCSSValue, CSSParser, Tokens} from "@internal/css-parser/types";
import {CSSCustomProperty, CSSFunction, CSSVarFunction} from "@internal/ast";
import {matchToken, nextToken} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";
import {parseComponentValue} from "@internal/css-parser/parser/value";

export function parseFunction(parser: CSSParser): CSSFunction | CSSVarFunction {
	const start = parser.getPosition();
	const token = parser.getToken() as Tokens["Function"];
	const name = token.value;
	const params: AnyCSSValue[] = [];
	const isVarFunction = name === "var";
	parser.nextToken();

	while (true) {
		if (matchToken(parser, "RightParen")) {
			nextToken(parser);
			break;
		}
		if (matchToken(parser, "EOF")) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_FUNCTION,
				token: parser.getToken(),
			});
			break;
		}
		const parsedValue = parseComponentValue(parser);
		if (parsedValue) {
			if (!params.length && isVarFunction) {
				if (parsedValue.type !== "CSSCustomProperty") {
					parser.unexpectedDiagnostic({
						description: descriptions.CSS_PARSER.INVALID_CUSTOM_PROPERTY,
						token: parser.getToken(),
					});
				}
				params.push(parsedValue);
			} else {
				params.push(parsedValue);
			}
		}
	}

	if (isVarFunction) {
		return parser.finishNode(
			start,
			{
				type: "CSSVarFunction",
				name,
				params: params as [CSSCustomProperty, ...AnyCSSValue[]],
			},
		);
	}

	return parser.finishNode(
		start,
		{
			type: "CSSFunction",
			name,
			params,
		},
	);
}
