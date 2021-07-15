import {CSSParser, Tokens} from "@internal/css-parser/types";
import {CSSString} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";
import {Position} from "@internal/parser-core";

let numberOfAreasFound: number;
let firstAreasPosition: Position;

/**
 * This function is responsible to parse the value of "grid-template-areas"
 * @param parser
 */
export function parseTemplateAreas(parser: CSSParser): CSSString | undefined {
	const currentToken = parser.getToken() as Tokens["String"];
	const start = parser.getPosition();
	const currentNumberOfAreas = currentToken.value.split(" ").length;

	if (numberOfAreasFound !== undefined) {
		if (currentNumberOfAreas !== numberOfAreasFound) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.GRID_TEMPLATE_INCORRECT_AREAS(
					numberOfAreasFound,
					parser.finishLocAt(firstAreasPosition, start),
				),
				token: currentToken,
			});
			return undefined;
		}
	} else {
		numberOfAreasFound = currentNumberOfAreas;
		firstAreasPosition = start;
	}

	return parser.finishNode(
		start,
		{
			type: "CSSString",
			value: currentToken.value,
		},
	);
}
