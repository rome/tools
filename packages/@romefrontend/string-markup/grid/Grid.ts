/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ChildNode,
	Children,
	GridOutputFormat,
	MarkupFormatGridOptions,
	MarkupLineWrapMode,
	MarkupPointer,
	MarkupTagName,
	TagNode,
	TextNode,
} from "../types";
import {
	Number1,
	ob1Add,
	ob1Coerce1,
	ob1Get,
	ob1Get1,
	ob1Inc,
	ob1Number1,
	ob1Sub,
} from "@romefrontend/ob1";
import {
	humanizeFileSize,
	humanizeNumber,
	humanizeTime,
} from "@romefrontend/string-utils";
import {
	buildFileLink,
	createEmptyAttributes,
	formatApprox,
	formatGrammarNumber,
	formatNumber,
} from "../util";
import {escapeXHTMLEntities} from "@romefrontend/html-parser";
import {ansiFormatText} from "./formatANSI";
import {htmlFormatText} from "./formatHTML";
import {
	TERMINAL_FEATURES_DEFAULT,
	TerminalFeatures,
} from "@romefrontend/environment";
import {parseMarkup} from "../parse";
import {Position} from "@romefrontend/parser-core";
import {lineWrapValidator} from "../tags";

type Cursor = {
	line: Number1;
	column: Number1;
};

function cursorToIndex(
	cursor: Cursor,
): {
	line: number;
	column: number;
} {
	return {
		line: ob1Get1(cursor.line) - 1,
		column: ob1Get1(cursor.column) - 1,
	};
}

type Ancestry = Array<TagNode>;

function createTag(
	name: TagNode["name"],
	attributes: TagNode["attributes"],
	children: TagNode["children"] = [],
): TagNode {
	return {
		type: "Tag",
		name,
		attributes,
		children,
	};
}

export default class Grid {
	constructor(opts: MarkupFormatGridOptions) {
		this.viewportWidth =
			opts.columns === undefined ? undefined : ob1Coerce1(opts.columns);
		this.markupOptions = opts;

		this.features =
			opts.features === undefined ? TERMINAL_FEATURES_DEFAULT : opts.features;

		this.pointer = opts.pointer;

		this.cursor = {
			line: ob1Number1,
			column: ob1Number1,
		};

		this.sourceCursor = {
			currentLineText: "",
			currentLine: ob1Number1,
			currentColumn: ob1Number1,
		};

		this.lineStartMeta = {
			indentationCount: 0,
			sourceColumn: ob1Number1,
		};

		const {lineWrapMode = "word-break"} = opts;
		this.lineWrapMode = lineWrapMode;
		this.width = ob1Number1;

		this.lines = [];
	}

	features: TerminalFeatures;
	lineWrapMode: MarkupLineWrapMode;
	markupOptions: MarkupFormatGridOptions;
	lines: Array<{
		ranges: Array<{
			start: number;
			end: number;
			ancestry: Ancestry;
		}>;
		columns: Array<string>;
	}>;
	pointer: undefined | MarkupPointer;
	cursor: Cursor;
	width: Number1;
	viewportWidth: undefined | Number1;

	// This tracks information about how much of the original source we have printed
	// We use this to point to and highlight certain sections. ie. drawPointer
	sourceCursor: {
		currentLineText: string;
		currentLine: Number1;
		currentColumn: Number1;
	};

	lineStartMeta: {
		indentationCount: number;
		sourceColumn: Number1;
	};

	alignRight() {
		const viewportWidth = ob1Get(this.viewportWidth);
		if (viewportWidth === undefined) {
			return;
		}

		this.lines = this.lines.map(({ranges, columns}) => {
			const newColumns = [...columns];

			// Pad out line to viewport width
			while (newColumns.length < viewportWidth) {
				newColumns.push(" ");
			}

			// Skip if all it contains is spaces

			let onlySpaces = true;
			for (const char of newColumns) {
				if (char !== " ") {
					onlySpaces = false;
				}
			}
			if (onlySpaces) {
				return {
					columns: newColumns,
					ranges,
				};
			}

			let offset = 0;

			// Shift whitespace from right to left
			while (newColumns[newColumns.length - 1] === " ") {
				offset++;
				newColumns.pop();
				newColumns.unshift(" ");
			}

			const newRanges = ranges.map((range) => {
				return {
					start: range.start + offset,
					end: range.end + offset,
					ancestry: range.ancestry,
				};
			});

			return {
				ranges: newRanges,
				columns: newColumns,
			};
		});
	}

	doesOverflowViewport(column: Number1): boolean {
		return (
			this.lineWrapMode !== "none" &&
			this.viewportWidth !== undefined &&
			ob1Get1(column) > ob1Get1(this.viewportWidth)
		);
	}

	getHeight(): Number1 {
		return ob1Coerce1(this.lines.length);
	}

	getLineWidth(lineIndex: number): number {
		const line = this.lines[lineIndex];
		return line === undefined ? 0 : line.columns.length;
	}

	getWidth(): Number1 {
		return this.width;
	}

	getSize(): {
		width: Number1;
		height: Number1;
	} {
		return {
			height: this.getHeight(),
			width: this.getWidth(),
		};
	}

	getCursor(): Cursor {
		return {...this.cursor};
	}

	getLines(format: GridOutputFormat): Array<string> {
		switch (format) {
			case "ansi":
				return this.getFormattedAnsiLines();

			case "html":
				return this.getFormattedHtmlLines();

			case "none":
				return this.getUnformattedLines();
		}
	}

	getUnformattedLines(): Array<string> {
		return this.lines.map(({columns}) => {
			return columns.join("").trimRight();
		});
	}

	getFormattedLines(
		opts: {
			normalizeText: (text: string) => string;
			formatTag: (tag: TagNode, inner: string) => string;
		},
	): Array<string> {
		const lines: Array<string> = [];

		for (const {ranges, columns} of this.lines) {
			// Sort ranges from last to first
			const sortedRanges = ranges.sort((a, b) => b.end - a.end);

			let line = "";

			let lastEnd: number | undefined = undefined;

			function catchUp(end: number) {
				line = `${columns.slice(end, lastEnd).join("")}${line}`;
				lastEnd = end;
			}

			for (const {start, end, ancestry} of sortedRanges) {
				catchUp(end);

				let substr = opts.normalizeText(columns.slice(start, end).join(""));

				// Format tags in reverse
				for (let i = ancestry.length - 1; i >= 0; i--) {
					const tag = ancestry[i];
					substr = opts.formatTag(tag, substr);
				}

				line = `${substr}${line}`;
				lastEnd = start;
			}

			catchUp(0);

			lines.push(line.trimRight());
		}

		return lines;
	}

	getFormattedHtmlLines(): Array<string> {
		return this.getFormattedLines({
			normalizeText: (text) => escapeXHTMLEntities(text),
			formatTag: (tag, inner) => htmlFormatText(tag, inner),
		});
	}

	getFormattedAnsiLines(): Array<string> {
		return this.getFormattedLines({
			normalizeText: (text) => text,
			formatTag: (tag, inner) => {
				return ansiFormatText(tag, inner, this.markupOptions, this.features);
			},
		});
	}

	fillCursor(cursor: Cursor) {
		const {line: lineIndex, column: colIndex} = cursorToIndex(cursor);

		// Pad lines
		for (let i = lineIndex; i >= 0 && this.lines[i] === undefined; i--) {
			this.lines[i] = {ranges: [], columns: []};
		}

		// Pad columns
		const line = this.lines[lineIndex];
		for (let i = colIndex - 1; i >= 0 && line.columns[i] === undefined; i--) {
			line.columns[i] = " ";
		}
	}

	moveCursor(cursor: Cursor) {
		if (cursor.line !== this.cursor.line) {
			this.lineStartMeta.indentationCount = 0;
			this.lineStartMeta.sourceColumn = this.sourceCursor.currentColumn;
		}

		this.cursor = cursor;
	}

	moveCursorBottomStart() {
		this.moveCursor({
			line: ob1Inc(this.getHeight()),
			column: ob1Number1,
		});
	}

	moveCursorRight(columns: Number1 = ob1Number1) {
		const newColumns = ob1Add(this.cursor.column, columns);

		// Perform character line wrap
		if (this.doesOverflowViewport(newColumns)) {
			const currentLine = this.lines[ob1Get1(this.cursor.line) - 1];

			this.moveCursorBottomStart();

			// Inherit the previous lines indentation
			if (currentLine !== undefined) {
				for (
					let i = 0;
					i < currentLine.columns.length && currentLine.columns[i] === " ";
					i++
				) {
					this.moveCursorRight();
					this.lineStartMeta.indentationCount++;
				}
			}
		} else {
			this.moveCursor({
				line: this.cursor.line,
				column: newColumns,
			});
		}
	}

	ensureNewline() {
		if (this.cursor.column !== ob1Number1) {
			this.newline();
		}
	}

	newline() {
		this.moveCursorBottomStart();
		this.fillCursor(this.cursor);
	}

	moveCursorStart() {
		this.moveCursor({
			line: this.cursor.line,
			column: ob1Number1,
		});
	}

	moveCursorDown(lines: Number1 = ob1Number1) {
		this.moveCursor({
			line: ob1Add(this.cursor.line, lines),
			column: this.cursor.column,
		});
	}

	writeToCursor(cursor: Cursor, char: string) {
		this.fillCursor(cursor);

		const {line: lineIndex, column: colIndex} = cursorToIndex(cursor);
		const line = this.lines[lineIndex];
		line.columns[colIndex] = char;

		if (cursor.column > this.width) {
			this.width = cursor.column;
		}
	}

	writeChar(char: string) {
		if (char === "\n") {
			this.newline();
			return;
		}

		this.writeToCursor(this.cursor, char);
		this.moveCursorRight();
	}

	drawText(tag: TextNode, ancestry: Ancestry) {
		this.writeText(tag.value, ancestry, tag.source);

		if (!tag.source && tag.sourceValue !== undefined) {
			for (const char of tag.sourceValue) {
				this.moveSourceCursor(char);
			}
		}
	}

	moveSourceCursor(char: string) {
		if (char === "\n") {
			this.sourceCursor.currentLineText = "";
			this.sourceCursor.currentColumn = ob1Number1;
			this.sourceCursor.currentLine = ob1Inc(this.sourceCursor.currentLine);
		} else {
			this.sourceCursor.currentLineText += char;
			this.sourceCursor.currentColumn = ob1Inc(this.sourceCursor.currentColumn);
		}
	}

	isInsidePointer(): boolean {
		const {sourceCursor, pointer} = this;

		if (pointer === undefined) {
			return false;
		}

		if (sourceCursor.currentLine !== pointer.line) {
			return false;
		}

		return (
			sourceCursor.currentColumn >= pointer.columnStart &&
			sourceCursor.currentColumn <= pointer.columnEnd
		);
	}

	writeText(text: string, ancestry: Ancestry, source: boolean) {
		if (text === "") {
			return;
		}

		const start = this.getCursor();

		const words = text.split(" ");

		let forceNextWordOverflow = false;

		for (let i = 0; i < words.length; i++) {
			const word = words[i];
			const isLastWord = i === words.length - 1;

			// Check if printing this word would overflow the viewport
			// If the whole word itself wouldn't fit on it's own line then we will
			// perform hard line wrapping in writeChar
			const willOverflow =
				this.lineWrapMode === "word-break" &&
				(forceNextWordOverflow ||
				(this.doesOverflowViewport(ob1Add(this.cursor.column, word.length - 1)) &&
				!this.doesOverflowViewport(ob1Coerce1(word.length))));
			if (willOverflow) {
				this.moveCursorBottomStart();
			}
			forceNextWordOverflow = false;

			for (const char of word.split(/(?:){1}/u)) {
				this.writeChar(char);

				if (source) {
					this.moveSourceCursor(char);
				}
			}

			let ignoreTrailingSpace = false;

			// Start of a sentence that was caused by line wrapping
			if (
				!word.endsWith("\n") &&
				this.cursor.column === ob1Number1 &&
				word !== ""
			) {
				ignoreTrailingSpace = true;
			}

			// If the next word will cause an overflow then don't print a leading space as it will be pointless
			const nextWord = words[i + 1];
			if (
				this.lineWrapMode === "word-break" &&
				nextWord !== undefined &&
				this.doesOverflowViewport(ob1Add(this.cursor.column, nextWord.length))
			) {
				ignoreTrailingSpace = true;
				forceNextWordOverflow = true;
			}

			if (isLastWord) {
				ignoreTrailingSpace = true;
			}

			if (!ignoreTrailingSpace) {
				this.writeChar(" ");
			}
		}

		const end = this.getCursor();
		this.addCursorRange(start, end, ancestry);
	}

	setRange(line: number, start: number, end: number, ancestry: Ancestry) {
		if (start === end) {
			// Nothing to format. Empty tag.
			return;
		}

		if (end < start) {
			throw new Error(
				`Range end for line index ${line} is before the start. end(${end}) < start(${start}). Line content: ${JSON.stringify(
					this.lines[line]?.columns,
				)}`,
			);
		}

		this.lines[line].ranges.push({
			start,
			end,
			ancestry,
		});
	}

	addCursorRange(_start: Cursor, _end: Cursor, ancestry: Ancestry) {
		if (ancestry.length === 0) {
			// No point storing a range without ancestry
			return;
		}

		const start = cursorToIndex(_start);
		const end = cursorToIndex(_end);

		if (start.line === end.line) {
			if (start.column === end.column) {
				// Empty range
				return;
			}

			this.setRange(start.line, start.column, end.column, ancestry);
		} else {
			// Add first line
			this.setRange(
				start.line,
				start.column,
				this.getLineWidth(start.line),
				ancestry,
			);

			// Add middle lines
			for (let line = start.line + 1; line < end.line; line++) {
				this.setRange(line, 0, this.getLineWidth(line), ancestry);
			}

			// Add last line
			this.setRange(end.line, 0, end.column, ancestry);
		}
	}

	drawList(tag: TagNode, ancestry: Ancestry) {
		let items: Array<TagNode> = [];
		for (const child of tag.children) {
			if (child.type === "Tag" && child.name === "li") {
				items.push(child);
			}
		}
		if (items.length === 0) {
			return;
		}

		this.ensureNewline();

		const ordered = tag.name === "ol";

		if (ordered) {
			const reversed = tag.attributes.get("reversed").asBoolean(false);
			const startOffset: number = tag.attributes.get("start").asNumber(0);

			const highestNumSize = humanizeNumber(items.length + startOffset).length;

			for (let i = 0; i < items.length; i++) {
				const item = items[i];

				let num = startOffset;
				if (reversed) {
					num += items.length - i;
				} else {
					num += i + 1;
				}

				const humanNum = humanizeNumber(num);
				const padding = " ".repeat(highestNumSize - humanNum.length);
				this.writeText(
					`${padding}${humanNum}. `,
					[createTag("dim", createEmptyAttributes())],
					false,
				);
				this.drawListItem(item, ancestry);
			}
		} else {
			for (const item of items) {
				this.writeText("- ", [createTag("dim", createEmptyAttributes())], false);
				this.drawListItem(item, ancestry);
			}
		}
	}

	getSubColumns(columns: Number1): undefined | number {
		return this.viewportWidth === undefined
			? undefined
			: ob1Get1(ob1Sub(this.viewportWidth, columns)) + 1;
	}

	drawListItem(item: TagNode, ancestry: Ancestry) {
		const grid = new Grid({
			...this.markupOptions,
			columns: this.getSubColumns(this.cursor.column),
		});
		grid.drawTag(item, ancestry);
		this.drawGrid(grid);
		this.moveCursorBottomStart();
	}

	drawPointer() {
		const {pointer, sourceCursor, lineStartMeta, cursor} = this;
		if (pointer === undefined) {
			return;
		}

		if (sourceCursor.currentLine !== pointer.line) {
			// I'm not quite sure what we are meant to do here
			return;
		}

		let start = ob1Get1(pointer.columnStart);
		let end = ob1Get1(pointer.columnEnd);

		if (cursor.line !== sourceCursor.currentLine) {
			start = 0;
			end = end - ob1Get1(lineStartMeta.sourceColumn);
		}

		let markerOffset = start;
		let markerSize = end - start;

		// Account for soft indentation
		markerOffset += lineStartMeta.indentationCount;

		// If the marker includes tabs then increase the size
		for (let i = start; i < end; i++) {
			if (sourceCursor.currentLineText[i] === "\t") {
				markerSize++;
			}
		}

		// If any previous text on this line contains tabs then increase the offset
		for (let i = 0; i < start; i++) {
			if (sourceCursor.currentLineText[i] === "\t") {
				markerOffset++;
			}
		}

		this.moveCursorBottomStart();

		// Pointer offset
		this.writeText(" ".repeat(markerOffset), [], false);

		// Pointer character
		if (pointer.char.length === 0) {
			this.writeText("^".repeat(markerSize), [], false);
		} else {
			for (let i = 0; i < markerSize; i++) {
				this.drawChildren(pointer.char, []);
			}
		}

		// Pointer message
		if (pointer.message.length > 0) {
			this.writeText(" ", [], false);
			this.drawView(
				{
					type: "Tag",
					name: "view",
					children: pointer.message,
					attributes: createEmptyAttributes(),
				},
				[],
			);
		}
	}

	parse(sub: string, offsetPosition: undefined | Position): Children {
		if (sub === "") {
			return [];
		}

		return this.normalizeChildren(
			parseMarkup(
				sub,
				{offsetPosition, sourceText: this.markupOptions.sourceText},
			),
		);
	}

	drawView(
		{children, attributes}: TagNode,
		ancestry: Ancestry,
		shrinkViewport: number = 0,
	) {
		// We allow markup in the linePrefix tag... Not sure how else we can support it.
		// NB: This assumes that the line prefix is only 1 height, maybe we could have some validation
		const linePrefix = this.parse(
			attributes.get("linePrefix").asString(""),
			attributes.get("linePrefix").getDiagnosticLocation("inner-value").start,
		);

		const linePrefixStart = this.getCursor();
		this.drawChildren(linePrefix, ancestry);
		const linePrefixEnd = this.getCursor();
		const hasLinePrefix =
			linePrefixStart.line !== linePrefixEnd.line ||
			linePrefixStart.column !== linePrefixEnd.column;

		// Calculate size of view
		let columns = this.getSubColumns(this.cursor.column);
		if (columns !== undefined) {
			columns -= shrinkViewport;
		}

		const pointer: MarkupPointer = {
			char: this.parse(
				attributes.get("pointerChar").asString(""),
				attributes.get("pointerChar").getDiagnosticLocation("inner-value").start,
			),
			message: this.parse(
				attributes.get("pointerMessage").asString(""),
				attributes.get("pointerMessage").getDiagnosticLocation("inner-value").start,
			),
			line: attributes.get("pointerLine").asOneIndexedNumber(0),
			columnStart: attributes.get("pointerStart").asOneIndexedNumber(0),
			columnEnd: attributes.get("pointerEnd").asOneIndexedNumber(0),
		};

		const lineWrapMode = lineWrapValidator(
			attributes.get("lineWrap").asStringOrVoid(),
		);
		const grid = new Grid({
			...this.markupOptions,
			pointer,
			columns,
			lineWrapMode,
		});
		grid.drawChildren(children, ancestry);
		grid.drawPointer();
		this.drawGrid(grid);

		if (hasLinePrefix) {
			// Add on any subsequent line prefixes if we wrapped
			for (let i = 1; i < ob1Get1(grid.getHeight()); i++) {
				this.moveCursor({
					line: ob1Add(linePrefixStart.line, i),
					column: linePrefixStart.column,
				});
				this.drawChildren(linePrefix, ancestry);
			}
		}

		this.moveCursor({
			line: this.getHeight(),
			column: linePrefixStart.column,
		});
	}

	drawTable(tag: TagNode, ancestry: Ancestry) {
		const rows: Array<Array<TagNode>> = [];

		for (const child of tag.children) {
			if (child.type === "Tag" && child.name === "tr") {
				const row: Array<TagNode> = [];

				for (const field of child.children) {
					if (field.type === "Tag" && field.name === "td") {
						row.push(field);
					} else {
						// Probably error?
					}
				}

				rows.push(row);
			} else {
				// Probably error?
			}
		}

		// Get the max number of columns for a row
		const columnCount = Math.max(...rows.map((columns) => columns.length));

		// Get column widths
		const columnWidths: Array<number> = [];
		for (let i = 0; i < columnCount; i++) {
			const widths = rows.map((row): number => {
				const field = row[i];
				if (field === undefined) {
					// Could be an excessive column
					return 0;
				} else {
					const grid = new Grid({...this.markupOptions, columns: undefined});
					grid.drawTag(field, ancestry);
					return ob1Get1(grid.getSize().width);
				}
			});
			columnWidths[i] = Math.max(...widths);
		}

		// If the column size exceed the stream columns then scale them all down
		const colsNeeded = columnWidths.reduce((a, b) => a + b, 0);
		const {viewportWidth} = this;
		let availableCols =
			viewportWidth === undefined
				? undefined
				: ob1Get(viewportWidth) - columnCount - 1;
		if (availableCols !== undefined && colsNeeded > availableCols) {
			// Find the biggest column
			let biggestColIndex = 0;
			for (let i = 0; i < columnWidths.length; i++) {
				const ourSize = columnWidths[i];
				const biggestSize = columnWidths[biggestColIndex];

				if (ourSize > biggestSize) {
					biggestColIndex = i;
				}
			}

			// Remove all columns from availableCols
			for (let i = 0; i < columnWidths.length; i++) {
				if (i !== biggestColIndex) {
					availableCols -= columnWidths[i];
				}
			}

			// Set biggest column to the availableCols
			columnWidths[biggestColIndex] = availableCols;
		}

		for (const row of rows) {
			for (let colIndex = 0; colIndex < row.length; colIndex++) {
				const field = row[colIndex];
				const width = ob1Coerce1(columnWidths[colIndex]);

				const grid = new Grid({...this.markupOptions, columns: ob1Get1(width)});
				grid.drawTag(field, ancestry);
				if (field.attributes.get("align").asStringOrVoid() === "right") {
					grid.alignRight();
				}

				this.drawGrid(grid);
				this.moveCursorRight(ob1Inc(width));
			}

			this.moveCursorBottomStart();
		}
	}

	drawGrid(grid: Grid) {
		const {lines} = grid;
		const cursor = cursorToIndex(this.getCursor());

		// Write
		for (let lineIndex = 0; lineIndex < lines.length; lineIndex++) {
			const {columns, ranges} = lines[lineIndex];

			const correctLine = cursor.line + lineIndex;

			for (let colIndex = 0; colIndex < columns.length; colIndex++) {
				const char = columns[colIndex];

				this.writeToCursor(
					{
						line: ob1Coerce1(correctLine + 1),
						column: ob1Coerce1(cursor.column + colIndex + 1),
					},
					char,
				);
			}

			for (const range of ranges) {
				this.setRange(
					correctLine,
					cursor.column + range.start,
					cursor.column + range.end,
					range.ancestry,
				);
			}
		}
	}

	drawTag(tag: TagNode, ancestry: Ancestry) {
		const hook = hooks.get(tag.name);

		const subAncestry: Ancestry = [...ancestry, tag];

		const oldLineWrapMode = this.lineWrapMode;

		if (tag.name === "nobr") {
			this.lineWrapMode = "none";
		}

		if (hook !== undefined && hook.before !== undefined) {
			hook.before(tag, this, ancestry);
		}

		switch (tag.name) {
			case "ol":
			case "ul": {
				this.drawList(tag, subAncestry);
				break;
			}

			case "table": {
				this.drawTable(tag, subAncestry);
				break;
			}

			case "view": {
				this.drawView(tag, subAncestry);
				break;
			}

			case "indent": {
				// Optimization for nested indents
				let levels = 1;
				let children: Children = tag.children;
				while (
					children.length === 1 &&
					children[0].type === "Tag" &&
					children[0].name === "indent"
				) {
					children = children[0].children;
					levels++;
				}

				for (let i = 0; i < levels; i++) {
					this.writeChar(" ");
					this.writeChar(" ");
				}

				this.drawView(
					createTag("view", createEmptyAttributes(), children),
					ancestry,
					levels * 2,
				);
				break;
			}

			default: {
				this.drawChildren(tag.children, subAncestry);
				break;
			}
		}

		if (hook !== undefined && hook.after !== undefined) {
			hook.after(tag, this, ancestry);
		}

		this.lineWrapMode = oldLineWrapMode;
	}

	drawChildren(children: Children, ancestry: Ancestry) {
		for (const child of children) {
			if (child.type === "Text") {
				this.drawText(child, ancestry);
			} else {
				this.drawTag(child, ancestry);
			}
		}
	}

	normalizeChildren(children: Children): Children {
		let newChildren: Children = [];

		for (const child of children) {
			newChildren = newChildren.concat(this.normalizeChild(child));
		}

		return newChildren;
	}

	normalizeChild(child: ChildNode): Children {
		if (child.type === "Text") {
			let {value} = child;

			if (value.includes("\t")) {
				const splitTabs = value.split("\t");
				const children: Children = [];

				for (let i = 0; i < splitTabs.length; i++) {
					if (i > 0) {
						children.push({
							type: "Text",
							source: false,
							sourceValue: "\t",
							value: "  ",
						});
					}

					const value = splitTabs[i];
					children.push({
						type: "Text",
						source: true,
						value,
					});
				}

				return children;
			}

			// Remove '\r' in case it snuck in as file contents
			value = value.replace(/\r/g, "");

			return [
				{
					type: "Text",
					source: true,
					value,
				},
			];
		}

		const tag = child;
		const children = this.normalizeChildren(tag.children);
		const textLength = getChildrenTextLength(children);
		const hasText = textLength > 0;

		let attributesWithoutEmphasis = tag.attributes;
		if (attributesWithoutEmphasis.has("emphasis")) {
			const emphasis = attributesWithoutEmphasis.get("emphasis").asBoolean(
				false,
			);
			attributesWithoutEmphasis = attributesWithoutEmphasis.copy({
				emphasis: undefined,
			});
			if (emphasis) {
				return this.normalizeChild(
					createTag(
						"emphasis",
						createEmptyAttributes(),
						[
							{
								...tag,
								attributes: attributesWithoutEmphasis,
							},
						],
					),
				);
			}
		}

		let attributes = attributesWithoutEmphasis;
		if (attributes.has("dim")) {
			const dim = attributes.get("dim").asBoolean(false);
			attributes = attributes.copy({dim: undefined});
			if (dim) {
				return this.normalizeChild(
					createTag(
						"dim",
						createEmptyAttributes(),
						[
							{
								...tag,
								attributes,
							},
						],
					),
				);
			}
		}

		// Insert padding
		if (tag.name === "pad") {
			const width = attributes.get("width").asNumber(0);
			const paddingSize = width - textLength;
			if (paddingSize > 0) {
				const paddingTextNode: TextNode = {
					type: "Text",
					source: false,
					value: " ".repeat(paddingSize),
				};
				if (tag.attributes.get("align").asStringOrVoid() === "right") {
					return [paddingTextNode, ...tag.children];
				} else {
					return [...tag.children, paddingTextNode];
				}
			} else {
				return tag.children;
			}
		}

		// Insert highlight legend
		if (tag.name === "highlight" && attributes.get("legend").asBoolean(false)) {
			const index = Math.min(0, attributes.get("i").asNumber(0));
			return [
				{
					...tag,
					attributes: attributes.copy({legend: undefined}),
				},
				createTag(
					"dim",
					createEmptyAttributes(),
					[
						{
							type: "Text",
							source: false,
							value: `[${String(index + 1)}]`,
						},
					],
				),
			];
		}

		if (hasText) {
			if (tag.name === "hr") {
				return [
					{
						...tag,
						children: [
							{
								type: "Text",
								source: false,
								value: " ",
							},
							...children,
							{
								type: "Text",
								source: false,
								value: " ",
							},
						],
					},
				];
			}
		} else {
			if (tag.name === "filelink") {
				return [
					{
						...tag,
						children: [
							{
								type: "Text",
								source: false,
								value: buildFileLink(tag.attributes, this.markupOptions).text,
							},
						],
					},
				];
			} else if (tag.name === "hyperlink") {
				return [
					{
						...tag,
						children: [
							{
								type: "Text",
								source: false,
								value: tag.attributes.get("target").asString(""),
							},
						],
					},
				];
			}
		}

		// These tags only expect text inside off them
		const singleInnerText =
			children.length === 1 && children[0].type === "Text"
				? children[0].value
				: undefined;
		if (singleInnerText !== undefined) {
			switch (tag.name) {
				case "filesize":
					return [
						{
							...tag,
							children: [
								{
									type: "Text",
									source: false,
									sourceValue: singleInnerText,
									value: humanizeFileSize(Number(singleInnerText)),
								},
							],
						},
					];

				case "duration":
					return [
						{
							...tag,
							children: [
								{
									type: "Text",
									source: false,
									sourceValue: singleInnerText,
									value: formatApprox(
										attributes,
										humanizeTime(Number(singleInnerText), true),
									),
								},
							],
						},
					];

				case "number":
					return [
						{
							...tag,
							children: [
								{
									type: "Text",
									source: false,
									sourceValue: singleInnerText,
									value: formatNumber(attributes, singleInnerText),
								},
							],
						},
					];

				case "grammarNumber":
					return [
						{
							...tag,
							children: [
								{
									type: "Text",
									source: false,
									sourceValue: singleInnerText,
									value: formatGrammarNumber(attributes, singleInnerText),
								},
							],
						},
					];
			}
		}

		return [
			{
				...tag,
				children,
			},
		];
	}
}

function getChildrenTextLength(children: Children): number {
	let length = 0;

	for (const child of children) {
		if (child.type === "Text") {
			length += child.value.length;
		}

		if (child.type === "Tag") {
			length += getChildrenTextLength(child.children);
		}
	}

	return length;
}

const hooks: Map<
	MarkupTagName,
	{
		before?: (tag: TagNode, grid: Grid, ancestry: Ancestry) => void;
		after?: (tag: TagNode, grid: Grid, ancestry: Ancestry) => void;
	}
> = new Map();

hooks.set(
	"hr",
	{
		after: (tag, grid, ancestry) => {
			let size =
				grid.viewportWidth === undefined
					? 100
					: ob1Get1(grid.viewportWidth) - ob1Get1(grid.cursor.column) + 1;
			size = Math.max(size, 0);
			grid.writeText("\u2501".repeat(size), ancestry, false);
		},
	},
);
