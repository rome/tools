import {MarkdownParser} from "@internal/markdown-parser";
import {
	AnyMarkdownInlineNode,
	MarkdownReference,
	MarkdownReferenceInline,
} from "@internal/ast";
import {parseTextWrapping} from "@internal/markdown-parser/parser/textwrapping";
import {descriptions} from "@internal/diagnostics";
import {parseText} from "@internal/markdown-parser/parser/text";

export function parseReference(
	parser: MarkdownParser,
): MarkdownReferenceInline | Array<AnyMarkdownInlineNode> {
	const pos = parser.getPosition();
	let reference: MarkdownReference | Array<AnyMarkdownInlineNode> = [];
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
					reference: (reference as MarkdownReference),
				},
			);
		}
		const token = parser.getToken();
		if (token.type === "Text") {
			reference.push(parseText(parser));
		}
		if (token.type === "Emphasis" || token.type === "Strong") {
			const textWrapping = parseTextWrapping(
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
			if (textWrapping) {
				reference.push(textWrapping);
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
		...(reference as Array<AnyMarkdownInlineNode>),
	];
}
