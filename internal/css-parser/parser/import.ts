import {CSSParser, Tokens} from "@internal/css-parser/types";
import {
	CSSImport,
	CSSAtImport
} from "@internal/ast";

export function parseImport(
	parser: CSSParser,
): CSSImport | undefined {
	const start = parser.getPosition();
	const value = parseAtImport(parser);

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
