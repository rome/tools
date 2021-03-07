import {TokenValues, isDigit} from "@internal/parser-core";
import {
	ListProperties,
	MarkdownParser,
	Tokens,
} from "@internal/markdown-parser";
import {ZeroIndexed} from "@internal/numbers";
import {MarkdownListChildren, MarkdownListItem} from "@internal/ast";
import {parseParagraph} from "./paragraph";

function isChecked(
	parser: MarkdownParser,
	index: ZeroIndexed,
): [boolean | undefined, ZeroIndexed] {
	const openSquareBracketChar = parser.getInputCharOnly(index.increment());
	const spaceOrXChar = parser.getInputCharOnly(index.add(2));
	const closedSquareBracketChar = parser.getInputCharOnly(index.add(3));

	if (openSquareBracketChar === "[" && closedSquareBracketChar === "]") {
		if (spaceOrXChar === " ") {
			return [false, index.add(3)];
		} else if (spaceOrXChar.toLowerCase() === "x") {
			return [true, index.add(3)];
		}
	}
	return [undefined, index];
}

export function tokenizeListItem(
	parser: MarkdownParser,
	index: ZeroIndexed,
	withChar?: undefined | "*" | "-",
): TokenValues<Tokens> | undefined {
	if (withChar) {
		const nextChar = parser.getInputCharOnly(index.increment());
		if (nextChar === " ") {
			const [checked, newIndex] = isChecked(parser, index.add(1));

			const endIndex = checked === undefined ? index.add(2) : newIndex.add(2);

			return parser.finishComplexToken<"ListItem", ListProperties>(
				"ListItem",
				{
					numeric: false,
					checked,
					value: withChar,
				},
				endIndex,
			);
		}
	}
	const [, endIndex] = parser.readInputFrom(index, isDigit);
	const nextChar = parser.getInputCharOnly(endIndex);
	const nextNextChar = parser.getInputCharOnly(endIndex.increment());
	if (nextChar === "." && (nextNextChar === " " || nextNextChar === "\n")) {
		return parser.finishComplexToken<"ListItem", ListProperties>(
			"ListItem",
			{
				numeric: true,
				checked: undefined,
			},
			endIndex.add(2),
		);
	}

	return undefined;
}

export function parseListItem(parser: MarkdownParser): MarkdownListItem {
	const token = parser.expectToken("ListItem");
	const pos = parser.getPosition();
	const children: MarkdownListChildren[] = [];

	while (
		!(parser.matchToken("EOF") ||
		parser.matchToken("NewLine") ||
		parser.matchToken("ListItem") ||
		parser.matchToken("Break"))
	) {
		children.push(parseParagraph(parser, true));
	}

	return parser.finishNode(
		pos,
		{
			// TODO handle check
			checked: token.checked,
			type: "MarkdownListItem",
			children,
			value: token.value,
		},
	);
}
