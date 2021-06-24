import {TokenValues, isDigit} from "@internal/parser-core";
import {
	ListProperties,
	MarkdownParser,
	Tokens,
} from "@internal/markdown-parser";
import {MarkdownListChildren, MarkdownListItem} from "@internal/ast";
import {parseParagraph} from "./paragraph";

function isChecked(tokenizer: MarkdownParser["tokenizer"]): boolean | undefined {
	if (tokenizer.eat("[ ]")) {
		tokenizer.eat(" ");
		return false;
	}

	if (tokenizer.eat("[x]") || tokenizer.eat("[X]")) {
		tokenizer.eat(" ");
		return true;
	}

	return undefined;
}

export function tokenizeListItem(
	parser: MarkdownParser,
	tokenizer: MarkdownParser["tokenizer"],
	withChar?: undefined | "*" | "-",
): TokenValues<Tokens> | undefined {
	if (withChar) {
		if (tokenizer.eat(`${withChar} `)) {
			const checked = isChecked(tokenizer);

			return tokenizer.finishComplexToken<"ListItem", ListProperties>(
				"ListItem",
				{
					numeric: false,
					checked,
					value: withChar,
				},
			);
		}
	} else {
		const start = tokenizer.index;
		tokenizer.read(isDigit);

		if (tokenizer.consume(".\n") || tokenizer.consume(". ")) {
			return tokenizer.finishComplexToken<"ListItem", ListProperties>(
				"ListItem",
				{
					numeric: true,
					checked: undefined,
				},
			);
		} else {
			tokenizer.setIndex(start);
		}
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
