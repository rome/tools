import {MarkdownParser} from "@internal/markdown-parser";
import {MarkdownReference, MarkdownReferenceInline} from "@internal/ast";
import {parseInline} from "@internal/markdown-parser/parser/inline";
import {descriptions} from "@internal/diagnostics";
import {parseText} from "@internal/markdown-parser/parser/text";

export function parseReference(parser: MarkdownParser): MarkdownReferenceInline {
	const pos = parser.getPosition();
	let reference: MarkdownReference = [];

	while (!parser.matchToken("EOF")) {
		const token = parser.getToken();
		if (token.type === "CloseSquareBracket") {
			parser.nextToken();
			break;
		}
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
						parser.unexpectedDiagnostic({
							description: descriptions.MARKDOWN_PARSER.ONLY_TEXT_INSIDE_DEFINITIONS,
						});
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

	return parser.finishNode(
		pos,
		{
			type: "MarkdownReferenceInline",
			value: "",
			reference,
		},
	);
}
