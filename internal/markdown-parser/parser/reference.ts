import {MarkdownParser} from "@internal/markdown-parser";
import {
	AnyMarkdownInlineNode,
	MarkdownReference,
	MarkdownReferenceInline,
} from "@internal/ast";
import {parseInline} from "@internal/markdown-parser/parser/inline";
import {descriptions} from "@internal/diagnostics";
import {parseText} from "@internal/markdown-parser/parser/text";

export function parseReference(
	parser: MarkdownParser,
): MarkdownReferenceInline | (AnyMarkdownInlineNode[]) {
	const pos = parser.getPosition();
	let reference: MarkdownReference | (AnyMarkdownInlineNode[]) = [];
	let unwantedTokens = false;

	while (!parser.matchToken("EOF")) {
		if (parser.matchToken("CloseSquareBracket")) {
			if (unwantedTokens) {
				parser.unexpectedDiagnostic({
					description: descriptions.MARKDOWN_PARSER.ONLY_TEXT_INSIDE_DEFINITIONS,
				});
			}
			parser.eatToken("CloseSquareBracket");
			return parser.finishNode(
				pos,
				{
					type: "MarkdownReferenceInline",
					value: "",
					reference: reference as MarkdownReference,
				},
			);
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
		parser.nextToken();
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
