import {parseInline} from "@internal/markdown-parser/parser/inline";
import {
	AnyMarkdownInlineNode,
	MarkdownTableCell,
	MarkdownTableRow,
} from "@internal/ast";

import {MarkdownTable} from "./../../ast/markdown/tables/MarkdownTable";
import {Number0, ob1Add} from "@internal/ob1";
import {TokenValues} from "@internal/parser-core";
import {MarkdownParser, Tokens} from "@internal/markdown-parser";

export function tokenizeTable(
	parser: MarkdownParser,
	index: Number0,
): TokenValues<Tokens> | undefined {
	const newIndex = ob1Add(index, 1);
	const [value, endIndexOfDelimiter] = parser.readInputFrom(
		newIndex,
		(char1) => char1 !== "|" && char1 !== "\n",
	);
	const delimiterChar = parser.getInputCharOnly(endIndexOfDelimiter);
	if (delimiterChar === "|") {
		return parser.finishToken("TablePipe", newIndex);
	} else if (delimiterChar === "\n" && value === "") {
		return parser.finishToken("TablePipe", endIndexOfDelimiter);
	}

	return undefined;
}

export function parseTable(parser: MarkdownParser): MarkdownTable {
	const rows: Array<MarkdownTableRow> = [];
	while (parser.matchToken("TablePipe")) {
		parser.nextToken();
		const cells: Array<MarkdownTableCell> = [];
		while (!parser.matchToken("NewLine")) {
			const startPos = parser.getPosition();
			const cellChildrens: Array<AnyMarkdownInlineNode> = [];
			while (!parser.matchToken("TablePipe")) {
				const inline = parseInline(parser);
				if (Array.isArray(inline)) {
					cellChildrens.push(...inline);
				} else if (inline !== undefined) {
					cellChildrens.push(inline);
				}
				parser.nextToken();
			}
			cells.push({
				type: "MarkdownTableCell",
				children: cellChildrens,
				loc: parser.finishLoc(startPos),
			});
			parser.nextToken();
		}
		rows.push({
			type: "MarkdownTableRow",
			children: cells,
		});
		parser.nextToken();
	}

	return {
		type: "MarkdownTable",
		children: rows,
	};
}
