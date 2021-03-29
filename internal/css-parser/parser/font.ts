import {CSSParser} from "@internal/css-parser/types";
import {CSSFontFace} from "@internal/ast/css/font/CSSFontFace";
import {matchToken, readToken} from "@internal/css-parser/tokenizer";
import {parseDeclarationBlock} from "@internal/css-parser/parser/declaration";

export function parseFontFace(parser: CSSParser): CSSFontFace | undefined {
	const start = parser.getPosition();

	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}

	const block = parseDeclarationBlock(parser);

	if (block) {
		return parser.finishNode(
			start,
			{
				type: "CSSFontFace",
				value: block,
			},
		);
	}

	return undefined;
}
