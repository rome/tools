import {AnyMarkdownInlineNode, MarkdownParagraph} from "@internal/ast";
import {MarkdownParser, isBlockToken} from "@internal/markdown-parser";
import {parseInline} from "@internal/markdown-parser/parser/inline";
import {descriptions} from "@internal/diagnostics";
import {parseText} from "@internal/markdown-parser/parser/text";
import {parseReference} from "@internal/markdown-parser/parser/reference";

export function parseParagraph(
	parser: MarkdownParser,
	isList?: boolean,
): MarkdownParagraph {
	const start = parser.getPosition();
	const children: AnyMarkdownInlineNode[] = [];
	while (!(parser.matchToken("EOF") || isBlockToken(parser))) {
		const token = parser.getToken();

		if (isList && token.type === "NewLine") {
			parser.nextToken();
			break;
		}
		switch (token.type) {
			case "Strong":
			case "Emphasis": {
				const nodes = parseInline(
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
				if (nodes) {
					children.push(nodes);
				}

				parser.nextToken();
				break;
			}
			case "Text": {
				children.push(parseText(parser));
				parser.nextToken();
				break;
			}
			case "NewLine": {
				const pos = parser.getPosition();
				children.push(
					parser.finishNode(
						pos,
						{
							type: "MarkdownText",
							value: "\n",
						},
					),
				);
				parser.nextToken();
				break;
			}
			case "OpenSquareBracket": {
				const reference = parseReference(parser);
				if (Array.isArray(reference)) {
					children.push(...reference);
				} else {
					children.push(reference);
				}
				// NOTE: do not go to the next token, parseReference already did that
				break;
			}
			default: {
				// TODO: to remove once all cases are handled
				parser.unexpectedDiagnostic({
					description: descriptions.MARKDOWN_PARSER.INVALID_SEQUENCE,
				});
				parser.nextToken();
			}
		}
	}

	return parser.finishNode(
		start,
		{
			type: "MarkdownParagraph",
			children,
		},
	);
}
