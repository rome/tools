import {CSSParser, Tokens} from "@internal/css-parser/types";
import {
	AnyCSSValue,
	CSSCalcFunction,
	CSSCustomProperty,
	CSSFunction,
	CSSUrlFunction,
	CSSVarFunction,
} from "@internal/ast";
import {matchToken, nextToken} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";
import {parseComponentValue} from "@internal/css-parser/parser/value";
import {parseCalcFunction} from "@internal/css-parser/parser/calc";

export function parseFunction(
	parser: CSSParser,
): CSSFunction | CSSVarFunction | CSSUrlFunction | CSSCalcFunction | undefined {
	const start = parser.getPosition();
	const token = parser.getToken() as Tokens["Function"];
	const name = token.value;
	const params: AnyCSSValue[] = [];
	const isVarFunction = name === "var";
	const isUrlFunction = name === "url";
	const isCalcFunction = name === "calc";
	nextToken(parser);

	if (isCalcFunction) {
		const value = parseCalcFunction(parser);
		if (value) {
			return value;
		}
	} else {
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

	if (isUrlFunction) {
		if (params.length > 1) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.URL_FUNCTION_TOO_MANY_PARAMETERS,
				token,
			});
		}
		const value = params[0];
		if (value.type === "CSSString") {
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

	return parser.finishNode(
		start,
		{
			type: "CSSFunction",
			name,
			params,
		},
	);
}
