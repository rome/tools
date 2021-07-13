import {CSSParser, Tokens} from "@internal/css-parser/types";
import {CSSMinmaxFunction, CSSMinmaxParam} from "@internal/ast";
import {nextToken, skipWhitespaces} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";

function parseArgument(
	parser: CSSParser,
	isFirst: boolean = false,
): CSSMinmaxParam | undefined {
	const token = parser.getToken();
	const start = parser.getPosition();
	if (token.type === "Dimension") {
		if (isFirst && token.unit === "fr") {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.MIN_MAX_INVALID_FLEX_ARGUMENT,
				token,
			});
			return undefined;
		}
		nextToken(parser);
		return parser.finishNode(
			start,
			{
				type: "CSSDimension",
				value: token.value,
				unit: token.unit,
			},
		);
	}
	if (token.type === "Percentage") {
		nextToken(parser);
		return parser.finishNode(
			start,
			{
				type: "CSSPercentage",
				value: token.value,
			},
		);
	}

	if (token.type === "Ident") {
		if (
			token.value !== "min-content" &&
			token.value !== "max-content" &&
			token.value !== "auto"
		) {
			nextToken(parser);
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.MIN_MAX_INVALID_ARGUMENTS,
				token,
			});
			return undefined;
		}
		nextToken(parser);
		return parser.finishNode(
			start,
			{
				type: "CSSRaw",
				value: token.value,
			},
		);
	}

	nextToken(parser);
	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.MIN_MAX_INVALID_ARGUMENTS,
		token,
	});

	return undefined;
}

export function parseMinmaxFunction(
	parser: CSSParser,
): undefined | CSSMinmaxFunction {
	// prepare variables needed for the final node
	const previousToken = parser.getPreviousToken() as Tokens["Ident"];
	const start = parser.getPositionFromIndex(previousToken.start);
	// starting by removing possible white spaces
	skipWhitespaces(parser);

	const firstArgument = parseArgument(parser, true);
	if (firstArgument) {
		skipWhitespaces(parser);
		const maybeComma = parser.eatToken("Comma");
		if (!maybeComma) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.MIN_MAX_INVALID_ARGUMENTS,
				token: parser.getToken(),
			});
			return undefined;
		}

		skipWhitespaces(parser);
		const secondArgument = parseArgument(parser);

		if (secondArgument) {
			skipWhitespaces(parser);
			const maybeParenToken = parser.eatToken("RightParen");

			if (!maybeParenToken) {
				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.MIN_MAX_INVALID_ARGUMENTS,
					token: maybeParenToken,
				});
				return undefined;
			}
			return parser.finishNode(
				start,
				{
					type: "CSSMinmaxFunction",
					params: [firstArgument, secondArgument],
					name: "minmax",
				},
			);
		}
	}

	return undefined;
}
