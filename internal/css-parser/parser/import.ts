import {CSSParser, Tokens} from "@internal/css-parser/types";

export function tryParseAtImport(parser: CSSParser): CSSAtImport | undefined {
	// css at import parser rules
}

export function parseImport(
	parser: CSSParser,
): CSSImport | undefined {
	const start = parser.getPosition();
	const value = tryParseAtImport(parser);

	if (value) {
		parser.nextToken();
		return parser.finishNode(
			start,
			{
				type: "CSSImport",
				name: "import",
				value,
			}
		);
	}

	return undefined;
}
