import {hardline} from "./../../../tokens";
import {ob1Get, ob1Sub} from "@internal/ob1";
import {MarkdownTable, MarkdownTableCell} from "@internal/ast";
import {Builder, Token, concat, join} from "@internal/formatter";

export default function MarkdownTable(
	builder: Builder,
	node: MarkdownTable,
): Token {
	const columnWidthMap: Record<number, number> = {};
	node.children.forEach((row) => {
		row.children.forEach((cell, index) => {
			const width = getCellWidth(cell);
			if (columnWidthMap[index] === undefined || columnWidthMap[index] < width) {
				columnWidthMap[index] = width;
			}
		});
	});

	const rowTokens = node.children.map((row) => {
		const cellTokens = row.children.map((cell, index) => {
			const contents = cell.children.map((child) =>
				builder.tokenize(child, cell)
			);
			return builder.alignCenter(concat(contents), columnWidthMap[index]);
		});
		return concat(["|", join("|", cellTokens), "|"]);
	});

	const tokens = concat([join(hardline, rowTokens), hardline]);

	return tokens;
}

function getCellWidth(cell: MarkdownTableCell): number {
	if (cell.loc === undefined) {
		return 0;
	}
	return ob1Get(ob1Sub(cell.loc.end.column, cell.loc.start.column));
}
