import {CSSParser, Tokens} from "@internal/css-parser/types";
import {CSSGridLine} from "@internal/ast";
import {nextToken, skipWhitespaces} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";

function parseSpan(parser: CSSParser): CSSGridLine | undefined {
	const spanPosition = parser.getPosition();
	const span = parser.getToken() as Tokens["Ident"];
	nextToken(parser);
	skipWhitespaces(parser);
	const maybeNumberOrIdent = parser.getToken();
	if (
		maybeNumberOrIdent.type !== "Number" &&
		maybeNumberOrIdent.type !== "Ident"
	) {
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.GRID_AREA_INCORRECT_SPAN,
			token: maybeNumberOrIdent,
		});
		return undefined;
	}
	const firstNode = parser.finishNode(
		spanPosition,
		{
			type: "CSSRaw",
			value: span.value,
		},
	);
	const secondPosition = parser.getPosition();
	if (maybeNumberOrIdent.type === "Number") {
		nextToken(parser);
		const secondNode = parser.finishNode(
			secondPosition,
			{
				type: "CSSNumber",
				value: maybeNumberOrIdent.value,
				raw: maybeNumberOrIdent.raw,
			},
		);
		return [firstNode, secondNode];
	}
	nextToken(parser);
	const secondNode = parser.finishNode(
		secondPosition,
		{
			type: "CSSRaw",
			value: maybeNumberOrIdent.value,
		},
	);
	return [firstNode, secondNode];
}

export function parseGridLine(parser: CSSParser): CSSGridLine | undefined {
	const token = parser.getToken();
	if (token.type === "Ident") {
		if (token.value === "span") {
			const spanValues = parseSpan(parser);
			if (spanValues) {
				return spanValues;
			}
			return undefined;
		}
		if (
			token.value === "auto" ||
			token.value === "inherit" ||
			token.value === "revert" ||
			token.value === "unset"
		) {
			const start = parser.getPosition();
			nextToken(parser);
			return [
				parser.finishNode(
					start,
					{
						type: "CSSRaw",
						value: token.value,
					},
				),
			];
		}
	}

	return undefined;
}
