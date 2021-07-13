import {AnyMarkdownInlineNode, MarkdownParagraph} from "@internal/ast";
import {MarkdownParser, isBlockToken} from "@internal/markdown-parser";
import {parseInline} from "@internal/markdown-parser/parser/inline";
import {descriptions} from "@internal/diagnostics";
import {parseText} from "@internal/markdown-parser/parser/text";
import {parseLink} from "@internal/markdown-parser/parser/link";
import {Position} from "@internal/parser-core";

export function parseParagraph(
	parser: MarkdownParser,
	isList?: boolean,
): MarkdownParagraph {
	const start = parser.getPosition();
	const children: AnyMarkdownInlineNode[] = [];
	let endPos: Position | null = null;

	while (!parser.matchToken("EOF")) {
		const token = parser.getToken();

		if (token.type === "NewLine") {
			if (isList) {
				break;
			}
			const currentPos = parser.getPosition();
			const next = parser.nextToken();
			if (next.type === "NewLine" || next.type === "EOF" || isBlockToken(next)) {
				// avoid including NewLine or EOF in paragraph
				endPos = currentPos;
				break;
			} else {
				children.push(
					parser.finishNode(
						currentPos,
						{
							type: "MarkdownText",
							value: "\n",
						},
					),
				);
				continue;
			}
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
							return parseLink(parser);
						}

						return parseText(parser);
					},
				);
				if (nodes) {
					children.push(nodes);
				}

				break;
			}

			case "Text": {
				children.push(parseText(parser));
				break;
			}

			case "OpenSquareBracket": {
				const reference = parseLink(parser);
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
					token,
				});
				parser.nextToken();
			}
		}
	}

	return parser.finishNodeAt(
		start,
		endPos || parser.getLastEndPosition(),
		{
			type: "MarkdownParagraph",
			children,
		},
	);
}
