import {MarkdownParser} from "@internal/markdown-parser";
import {
	AnyMarkdownInlineNode,
	MarkdownLink,
	MarkdownReference,
} from "@internal/ast";
import {parseInline} from "@internal/markdown-parser/parser/inline";
import {descriptions} from "@internal/diagnostics";
import {parseText} from "@internal/markdown-parser/parser/text";

export function parseLink(
	parser: MarkdownParser,
): MarkdownLink | AnyMarkdownInlineNode[] {
	parser.expectToken("OpenSquareBracket");

	const pos = parser.getPosition();
	let reference: MarkdownReference | AnyMarkdownInlineNode[] = [];
	let unwantedTokens = false;

	while (!parser.matchToken("EOF")) {
		if (parser.matchToken("CloseSquareBracket")) {
			if (unwantedTokens) {
				parser.unexpectedDiagnostic({
					description: descriptions.MARKDOWN_PARSER.ONLY_TEXT_INSIDE_DEFINITIONS,
				});
			}

			const closeSquarePos = parser.getPosition();
			parser.eatToken("CloseSquareBracket");
			if (parser.getToken().type === "OpenBracket") {
				parser.eatToken("OpenBracket");
				const link = parser.eatToken("Text");
				const closeBracket = parser.eatToken("CloseBracket");
				if (link && closeBracket) {
					return parser.finishNode(
						pos,
						{
							type: "MarkdownLink",
							text: reference as MarkdownReference,
							link: link.value,
						},
					);
				} else {
					reference.push(
						parser.finishNode(
							pos,
							{
								type: "MarkdownText",
								value: `(${link?.value}`,
							},
						),
					);
					return [...(reference as AnyMarkdownInlineNode[])];
				}
			} else {
				return [
					parser.finishNode(
						pos,
						{
							type: "MarkdownText",
							value: "[",
						},
					),
					...(reference as AnyMarkdownInlineNode[]),
					parser.finishNode(
						closeSquarePos,
						{
							type: "MarkdownText",
							value: "]",
						},
					),
				];
			}
		}
		const token = parser.getToken();
		if (token.type === "Text") {
			reference.push(parseText(parser));
		}
		if (token.type === "Emphasis" || token.type === "Strong") {
			const inline = parseInline(
				parser,
				token,
				(unknownToken) => {
					if (unknownToken.type === "CloseSquareBracket") {
						return undefined;
					}
					if (unknownToken.type !== "Text") {
						unwantedTokens = true;
					}

					return parseText(parser);
				},
			);
			if (inline) {
				reference.push(inline);
			}
		}
	}

	return [
		parser.finishNode(
			pos,
			{
				type: "MarkdownText",
				value: "[",
			},
		),
		...(reference as AnyMarkdownInlineNode[]),
	];
}
