import {MarkdownText} from "@internal/ast";
import {MarkdownParser} from "@internal/markdown-parser";
import {descriptions} from "@internal/diagnostics";

export function parseText(parser: MarkdownParser): MarkdownText {
	const token = parser.getToken();
	const pos = parser.getPosition();
	if (token.type === "Text") {
		parser.nextToken();
		return parser.finishNode(
			pos,
			{
				type: "MarkdownText",
				value: token.value,
			},
		);
	}
	parser.unexpectedDiagnostic({
		description: descriptions.MARKDOWN_PARSER.TEXT_WITHOUT_TEXT,
	});
	return parser.finishNode(
		pos,
		{
			type: "MarkdownText",
			value: "",
		},
	);
}
