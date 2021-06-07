import {CSSParser} from "@internal/css-parser/types";
import {
	CSSImport
} from "@internal/ast";
import {parseAtImport} from "@internal/css-parser/parser/at-import";
import {nextToken} from "@internal/css-parser/tokenizer";

export function parseImport(
	parser: CSSParser,
): CSSImport | undefined {
	const start = parser.getPosition();
	const value = parseAtImport({parser});

	if (value) {
		nextToken(parser);
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
