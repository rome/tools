import { MarkdownParagraph } from './../../ast/markdown/core/MarkdownParagraph';
import { MarkdownTableCell } from '@internal/ast';
import { parseParagraph } from '@internal/markdown-parser/parser/paragraph';
import { MarkdownTable } from './../../ast/markdown/tables/MarkdownTable';
import { Number0, ob1Add } from "@internal/ob1";
import { TokenValues } from "@internal/parser-core";
import {
	MarkdownParser,
	Tokens,
} from "@internal/markdown-parser";
import { MarkdownTableRow } from '@internal/ast';

export function tokenizeTable(
	parser: MarkdownParser,
	index: Number0,
): TokenValues<Tokens> | undefined {
  const newIndex = ob1Add(index, 1);
  const [value, endIndexOfDelimiter] = parser.readInputFrom(
		newIndex,
		(char1) => char1 !== '|' && char1 !== "\n",
  );
  const delimiterChar = parser.getInputCharOnly(endIndexOfDelimiter);
  if (delimiterChar === '|') {
    return parser.finishToken("TablePipe", newIndex);
  } else if(delimiterChar === '\n' && value === '') {
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
      const cellChildrens: Array<MarkdownParagraph> = [];
      while (!parser.matchToken("TablePipe")) {
        cellChildrens.push(parseParagraph(parser, false, true))
      }
      cells.push({
        type: "MarkdownTableCell",
        children: cellChildrens
      })
      parser.nextToken();
    }
    rows.push({
      type: "MarkdownTableRow",
      children: cells
    })
    parser.nextToken();
  }

  return {
    type: "MarkdownTable",
    children: rows
  }
}
