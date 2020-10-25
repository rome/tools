import {parseInline} from "@internal/markdown-parser/parser/inline";
import {AnyMarkdownInlineNode, MarkdownParagraph} from "@internal/ast";
import {MarkdownParser, isBlockToken} from "@internal/markdown-parser";
import {descriptions} from "@internal/diagnostics";

export function parseParagraph(
	parser: MarkdownParser,
	isList?: boolean,
	isTable?: boolean,
): MarkdownParagraph {
	const start = parser.getPosition();
	const children: Array<AnyMarkdownInlineNode> = [];
	while (!parser.matchToken("EOF") && !isBlockToken(parser)) {
		const token = parser.getToken();

		if (isList && token.type === "NewLine") {
			parser.nextToken();
			break;
		}
		if (isTable && token.type === "TablePipe") {
			break;
		}

		const inline = parseInline(parser);
		if (inline === undefined) {
			// TODO: to remove once all cases are handled
			parser.unexpectedDiagnostic({
				description: descriptions.MARKDOWN_PARSER.INVALID_SEQUENCE,
			});
		} else if (Array.isArray(inline)) {
			children.push(...inline);
		} else {
			children.push(inline);
		}

		parser.nextToken();
	}

	return parser.finishNode(
		start,
		{
			type: "MarkdownParagraph",
			children,
		},
	);
}
