import {CSSParser} from "@internal/css-parser/types";
import {CSSGridTemplateAreaValue, CSSRaw, CSSString} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";
import {Position} from "@internal/parser-core";
import {GRID_GLOBAL_VALUES} from "@internal/css-parser/parser/grid/shared";
import {
	matchToken,
	nextToken,
	skipWhitespaces,
} from "@internal/css-parser/tokenizer";

/**
 * This function is responsible to parse the value of "grid-template-areas"
 * @param parser
 */
export function parseTemplateAreas(
	parser: CSSParser,
): CSSGridTemplateAreaValue | undefined {
	let numberOfAreasFound: number | undefined = undefined;
	let firstAreasPosition: Position | undefined = undefined;
	let value: Array<CSSRaw | CSSString> = [];
	const start = parser.getPosition();

	while (!(matchToken(parser, "EOF") || matchToken(parser, "Semi"))) {
		skipWhitespaces(parser);
		const currentToken = parser.getToken();
		const start = parser.getPosition();

		if (currentToken.type === "String") {
			const currentNumberOfAreas = currentToken.value.split(" ").length;

			if (
				numberOfAreasFound !== undefined &&
				firstAreasPosition !== undefined &&
				currentNumberOfAreas !== numberOfAreasFound
			) {
				nextToken(parser);
				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.GRID_TEMPLATE_INCORRECT_AREAS(
						numberOfAreasFound,
						parser.finishLocAt(firstAreasPosition, start),
					),
					token: currentToken,
				});
				return undefined;
			} else {
				numberOfAreasFound = currentNumberOfAreas;
				firstAreasPosition = start;
			}

			nextToken(parser);
			value.push(
				parser.finishNode(
					start,
					{
						type: "CSSString",
						value: currentToken.value,
					},
				),
			);
		} else if (currentToken.type === "Ident") {
			if (GRID_GLOBAL_VALUES.includes(currentToken.value)) {
				nextToken(parser);
				value.push(
					parser.finishNode(
						start,
						{
							type: "CSSRaw",
							value: currentToken.value,
						},
					),
				);
			} else {
				nextToken(parser);
				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.GRID_INVALID_GLOBAL_VALUE(
						GRID_GLOBAL_VALUES,
					),
					token: currentToken,
				});
				return undefined;
			}
		} else {
			nextToken(parser);
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.GRID_TEMPLATE_UNSUPPORTED_VALUE,
				token: currentToken,
			});
			return undefined;
		}
	}
	if (value) {
		return parser.finishNode(
			start,
			{
				type: "CSSGridTemplateAreaValue",
				value,
			},
		);
	}
	return undefined;
}
