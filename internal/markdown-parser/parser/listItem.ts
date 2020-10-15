import {TokenValues, isDigit} from "@internal/parser-core";
import {
	ListProperties,
	MarkdownParser,
	Tokens,
} from "@internal/markdown-parser";
import {Number0, ob1Add} from "@internal/ob1";
import {MarkdownListChildren, MarkdownListItem} from "@internal/ast";
import {parseParagraph} from "./paragraph";

function isChecked(
	parser: MarkdownParser,
	index: Number0,
): [boolean | undefined, Number0] {
	const openSquareBracketChar = parser.getInputCharOnly(index, 1);
	const spaceOrXChar = parser.getInputCharOnly(index, 2);
	const closedSquareBracketChar = parser.getInputCharOnly(index, 3);

	if (openSquareBracketChar === "[" && closedSquareBracketChar === "]") {
		if (spaceOrXChar === " ") {
			return [false, ob1Add(index, 3)];
		} else if (spaceOrXChar.toLowerCase() === "x") {
			return [true, ob1Add(index, 3)];
		}
	}
	return [undefined, index];
}

export function tokenizeListItem(
	parser: MarkdownParser,
	index: Number0,
	withChar?: undefined | "*" | "-",
): TokenValues<Tokens> | undefined {
	if (withChar) {
		const nextChar = parser.getInputCharOnly(index, 1);
		if (nextChar === " ") {
			const [checked, newIndex] = isChecked(parser, ob1Add(index, 1));

			const endIndex =
				checked === undefined ? ob1Add(index, 2) : ob1Add(newIndex, 2);

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
	const nextNextChar = parser.getInputCharOnly(endIndex, 1);
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

export function parseListItem(parser: MarkdownParser): MarkdownListItem {
	const token = parser.expectToken("ListItem");
	const pos = parser.getPosition();
	const children: Array<MarkdownListChildren> = [];

	while (
		!parser.matchToken("EOF") &&
		!parser.matchToken("NewLine") &&
		!parser.matchToken("ListItem") &&
		!parser.matchToken("Break")
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
