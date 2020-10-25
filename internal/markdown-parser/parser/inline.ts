import {AnyMarkdownInlineNode} from "@internal/ast";
import {MarkdownParser} from "@internal/markdown-parser";
import {parseTextWrapping} from "@internal/markdown-parser/parser/textwrapping";
import {parseText} from "@internal/markdown-parser/parser/text";
import {parseReference} from "@internal/markdown-parser/parser/reference";

export function parseInline(
	parser: MarkdownParser,
): AnyMarkdownInlineNode | Array<AnyMarkdownInlineNode> | undefined {
	const token = parser.getToken();

	switch (token.type) {
		case "Strong":
		case "Emphasis": {
			const nodes = parseTextWrapping(
				parser,
				token,
				// TODO: to add support for more inline tokens: link, code inline block
				(unknownToken) => {
					if (unknownToken.type === "OpenSquareBracket") {
						return parseReference(parser);
					}
					return parseText(parser);
				},
			);
			return nodes;
		}
		case "Text": {
			return parseText(parser);
		}
		case "NewLine": {
			const pos = parser.getPosition();
			return parser.finishNode(
				pos,
				{
					type: "MarkdownText",
					value: "\n",
				},
			);
		}
		case "OpenSquareBracket": {
			return parseReference(parser);
		}
		default: {
			return undefined;
		}
	}
}
