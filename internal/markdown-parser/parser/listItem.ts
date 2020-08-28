import {TokenValues, isDigit} from "@internal/parser-core";
import {
	ListProperties,
	MarkdownParser,
	Tokens,
} from "@internal/markdown-parser";
import {Number0, ob1Add} from "@internal/ob1";

export function tokenizeListItem(
	parser: MarkdownParser,
	index: Number0,
	withChar?: undefined | "*" | "-",
): TokenValues<Tokens> | undefined {
	const [, endIndex] = parser.readInputFrom(index, isDigit);
	const nextChar = parser.getInputCharOnly(endIndex);
	const nextNextChar = parser.getInputCharOnly(endIndex, 1);

	if (withChar) {
		const nextChar = parser.getInputCharOnly(index, 1);
		if (nextChar === " ") {
			return parser.finishComplexToken<"ListItem", ListProperties>(
				"ListItem",
				{
					numeric: false,
					checked: undefined,
					value: withChar,
				},
				ob1Add(index, 2),
			);
		}
	}
	if (nextChar === "." && (nextNextChar === " " || nextNextChar === "\n")) {
		return parser.finishComplexToken<"ListItem", ListProperties>(
			"ListItem",
			{
				numeric: true,
				checked: undefined,
			},
			ob1Add(endIndex, 2),
		);
	}

	return undefined;
}
