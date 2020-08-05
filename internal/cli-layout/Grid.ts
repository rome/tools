/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyMarkup,
	MarkupLineWrapMode,
	MarkupParsedChild,
	MarkupParsedChildren,
	MarkupParsedTag,
	MarkupParsedText,
	MarkupTagName,
	buildFileLink,
	createEmptyAttributes,
	formatApprox,
	formatGrammarNumber,
	formatNumber,
	lineWrapValidator,
	parseMarkup,
} from "@internal/markup";
import {GridOptions, GridOutputFormat, GridPointer} from "./types";
import {
	Number1,
	ob1Add,
	ob1Coerce1,
	ob1Dec,
	ob1Get,
	ob1Get1,
	ob1Inc,
	ob1Number1,
	ob1Sub,
} from "@internal/ob1";
import {
	humanizeFileSize,
	humanizeNumber,
	humanizeTime,
	splitChars,
} from "@internal/string-utils";
import {escapeXHTMLEntities} from "@internal/html-parser";
import {ansiFormatText} from "./formatANSI";
import {htmlFormatText} from "./formatHTML";
import {
	DEFAULT_TERMINAL_FEATURES,
	TerminalFeatures,
} from "@internal/cli-environment";
import {Position} from "@internal/parser-core";
import {formatAnsi} from "./ansi";
import {pretty} from "@internal/pretty-format";

type Cursor = {
	line: Number1;
	column: Number1;
};

type Rows = Array<Array<MarkupParsedTag>>;

type Ancestry = Array<MarkupParsedTag>;

type Column = string | symbol;
type Columns = Array<Column>;

function createTag(
	name: MarkupParsedTag["name"],
	attributes: MarkupParsedTag["attributes"] = createEmptyAttributes(),
	children: MarkupParsedTag["children"] = [],
): MarkupParsedTag {
	return {
		type: "Tag",
		name,
		attributes,
		children,
	};
}

function joinColumns(columns: Columns): string {
	return columns.filter((column) => typeof column === "string").join("");
}

function sliceColumns(columns: Columns, start: Number1, end: Number1): string {
	return joinColumns(columns.slice(ob1Get1(start) - 1, ob1Get1(end) - 1));
}

function extractViewTags(
	tag: MarkupParsedTag,
): {
	pointer: undefined | MarkupParsedTag;
	linePrefixes: Array<MarkupParsedTag>;
	children: MarkupParsedChildren;
} {
	let pointer: undefined | MarkupParsedTag;
	let linePrefixes: Array<MarkupParsedTag> = [];
	let children: MarkupParsedChildren = [];

	for (const child of tag.children) {
		if (child.type === "Tag") {
			if (child.name === "viewLinePrefix") {
				linePrefixes.push(child);
				continue;
			} else if (child.name === "viewPointer") {
				pointer = child;
				continue;
			}
		}

		children.push(child);
	}

	return {pointer, linePrefixes, children};
}

type GridLine = {
	ranges: Array<{
		start: Number1;
		end: Number1;
		ancestry: Ancestry;
	}>;
	columns: Columns;
};

const TAB_PLACEHOLDER_COLUMN = Symbol("TAB_LAYOUT_PLACEHOLDER");

function isWhitespace(char: Column): boolean {
	return char === " " || char === "\t" || char === TAB_PLACEHOLDER_COLUMN;
}

export default class Grid {
	constructor(opts: GridOptions) {
		this.viewportWidth = opts.columns === undefined ? undefined : opts.columns;
		this.options = opts;

		this.features =
			opts.features === undefined ? DEFAULT_TERMINAL_FEATURES : opts.features;

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
			softWrapped: false,
			indentationCount: 0,
			sourceColumn: ob1Number1,
		};

		const {lineWrapMode = "word-break"} = opts.view;
		this.lineWrapMode = lineWrapMode;
		this.width = ob1Number1;

		this.lines = [
			// First line that we're going to fill.
			// If it isn't here then any height measurements before anything has been written will be wrong
			{
				ranges: [],
				columns: [],
			},
		];

		// TODO make the tab width customizable in userConfig
		this.tabSize = 2;

		const indentColumns: Columns = [];
		for (let i = 0; i < this.tabSize; i++) {
			let char: Column = " ";
			if (!this.options.convertTabs) {
				char = i === 0 ? "\t" : TAB_PLACEHOLDER_COLUMN;
			}
			indentColumns.push(char);
		}
		this.indentColumns = indentColumns;
	}

	private indentColumns: Columns;
	private tabSize: number;

	public features: TerminalFeatures;
	private lineWrapMode: MarkupLineWrapMode;
	public options: GridOptions;
	private lines: Array<GridLine>;
	public cursor: Cursor;
	private width: Number1;
	public viewportWidth: undefined | Number1;

	// This tracks information about how much of the original source we have printed
	// We use this to point to and highlight certain sections. ie. drawPointer
	private sourceCursor: {
		currentLineText: string;
		currentLine: Number1;
		currentColumn: Number1;
	};

	private lineStartMeta: {
		softWrapped: boolean;
		indentationCount: number;
		sourceColumn: Number1;
	};

	private debugState(): object {
		return {
			cursor: this.cursor,
			width: this.width,
			height: this.getHeight(),
			lines: this.lines.map(({columns, ranges}) => {
				return {
					line: joinColumns(columns),
					ranges: ranges.map(({ancestry, start, end}) => ({
						start,
						end,
						ancestry: ancestry.map((tag) => tag.name).join("|"),
					})),
				};
			}),
		};
	}

	private alignRight() {
		const viewportWidth = ob1Get(this.viewportWidth);
		if (viewportWidth === undefined) {
			return;
		}

		this.lines = this.lines.map(({ranges, columns}) => {
			let newColumns = [];

			// Pad out line to viewport width
			const offset = Math.max(0, viewportWidth - columns.length);
			for (const char of this.getSpaces(offset)) {
				newColumns.push(char);
			}

			newColumns = newColumns.concat(columns);

			// Skip if all it contains is whitespace
			let onlyWhitespace = true;
			for (const char of newColumns) {
				if (!isWhitespace(char)) {
					onlyWhitespace = false;
				}
			}
			if (onlyWhitespace) {
				return {
					columns: newColumns,
					ranges,
				};
			}

			const newRanges = ranges.map((range) => {
				return {
					start: ob1Add(range.start, offset),
					end: ob1Add(range.end, offset),
					ancestry: range.ancestry,
				};
			});

			return {
				ranges: newRanges,
				columns: newColumns,
			};
		});
	}

	private doesOverflowViewport(column: Number1): boolean {
		return (
			this.lineWrapMode !== "none" &&
			this.viewportWidth !== undefined &&
			ob1Get1(column) > ob1Get1(this.viewportWidth)
		);
	}

	private getHeight(): Number1 {
		return ob1Coerce1(this.lines.length);
	}

	private getLine(line: Number1): GridLine {
		const lineIndex = ob1Get1(line) - 1;
		for (let i = lineIndex; i >= 0 && this.lines[i] === undefined; i--) {
			this.clearLine(ob1Coerce1(i + 1));
		}
		return this.lines[lineIndex];
	}

	private clearLine(line: Number1) {
		this.lines[ob1Get1(line) - 1] = {ranges: [], columns: []};
	}

	private getLineWidth(line: Number1): Number1 {
		return ob1Coerce1(this.getLine(line).columns.length);
	}

	public getWidth(): Number1 {
		return this.width;
	}

	private getSize(): {
		width: Number1;
		height: Number1;
	} {
		return {
			height: this.getHeight(),
			width: this.getWidth(),
		};
	}

	private getCursor(): Cursor {
		return {...this.cursor};
	}

	public getLines(format: GridOutputFormat): Array<string> {
		switch (format) {
			case "ansi":
				return this.getFormattedAnsiLines();

			case "html":
				return this.getFormattedHtmlLines();

			case "none":
				return this.getUnformattedLines();
		}
	}

	private getTrimmedLines(): Array<GridLine> {
		const lines = [...this.lines];

		// Remove empty columns
		// Explicit newlines will have at least one column with an empty field
		while (lines.length > 0 && lines[lines.length - 1].columns.length === 0) {
			lines.pop();
		}

		return lines;
	}

	private getUnformattedLines(): Array<string> {
		return this.lines.map(({columns}) => {
			return joinColumns(columns).trimRight();
		});
	}

	private getFormattedLines(
		opts: {
			normalizeText: (text: string) => string;
			formatTag: (tag: MarkupParsedTag, inner: string) => string;
			wrapRange: (text: string) => string;
		},
	): Array<string> {
		const lines: Array<string> = [];

		for (const {ranges, columns} of this.getTrimmedLines()) {
			// Sort ranges from last to first
			const sortedRanges = ranges.sort((a, b) => ob1Get1(b.end) - ob1Get1(a.end));

			let line = "";

			let lastEnd: number | undefined = undefined;

			function catchUp(end: number) {
				const start = opts.normalizeText(
					joinColumns(columns.slice(end, lastEnd)),
				);
				line = `${start}${line}`;
				lastEnd = end;
			}

			for (const {start, end, ancestry} of sortedRanges) {
				catchUp(ob1Get1(end) - 1);

				let substr = opts.normalizeText(sliceColumns(columns, start, end));

				// Format tags in reverse
				for (let i = ancestry.length - 1; i >= 0; i--) {
					const tag = ancestry[i];
					substr = opts.formatTag(tag, substr);
				}

				substr = opts.wrapRange(substr);

				line = `${substr}${line}`;
				lastEnd = ob1Get1(start) - 1;
			}

			catchUp(0);

			lines.push(line.trimRight());
		}

		return lines;
	}

	private getFormattedHtmlLines(): Array<string> {
		return this.getFormattedLines({
			normalizeText: (text) => escapeXHTMLEntities(text),
			formatTag: (tag, inner) => htmlFormatText(tag, inner),
			wrapRange: (str) => str,
		});
	}

	private getFormattedAnsiLines(): Array<string> {
		return this.getFormattedLines({
			normalizeText: (text) => text,
			formatTag: (tag, inner) => {
				return ansiFormatText(tag, inner, this);
			},
			wrapRange: (str) => formatAnsi.reset(str),
		});
	}

	// Fill current cursor line with spaces until the column
	private fillCursor(cursor: Cursor) {
		const line = this.getLine(cursor.line);
		const colIndex = ob1Get1(cursor.column) - 1;

		// Amount of spaces to insert
		let count = 0;

		// Calculate how many we need to insert
		let i = colIndex - 1;
		while (i >= 0 && (line.columns[i] === undefined || line.columns[i] === "")) {
			i--;
			count++;
		}

		// Insert spaces
		for (const char of this.getSpaces(count)) {
			i++;
			line.columns[i] = char;
		}
	}

	private moveCursor(cursor: Cursor) {
		if (cursor.line !== this.cursor.line) {
			this.lineStartMeta.softWrapped = false;
			this.lineStartMeta.indentationCount = 0;
			this.lineStartMeta.sourceColumn = this.sourceCursor.currentColumn;
		}

		this.cursor = cursor;
	}

	private moveCursorRight(columns: Number1 = ob1Number1) {
		const newColumns = ob1Add(this.cursor.column, columns);

		// Perform character line wrap
		if (this.doesOverflowViewport(newColumns)) {
			const currentLine = this.getLine(this.cursor.line);
			const previousLineSoftWrapped = this.lineStartMeta.softWrapped;

			this.newline();

			// Soft wrap, inherit the previous lines indentation
			if (currentLine !== undefined) {
				let i = 0;
				while (
					i < currentLine.columns.length &&
					isWhitespace(currentLine.columns[i])
				) {
					this.moveCursorRight();
					this.lineStartMeta.indentationCount++;
					i++;
				}

				const {extraSoftWrapIndent} = this.options.view;
				if (previousLineSoftWrapped) {
					this.lineStartMeta.softWrapped = true;
				} else if (extraSoftWrapIndent !== undefined) {
					this.lineStartMeta.softWrapped = true;
					for (let i = 0; i < extraSoftWrapIndent; i++) {
						this.moveCursorRight();
					}
				}
			}
		} else {
			this.moveCursor({
				line: this.cursor.line,
				column: newColumns,
			});
		}
	}

	private userNewline() {
		this.newline();
		this.writeToCursor(this.cursor, "");
	}

	private newline() {
		this.moveCursor({
			line: ob1Inc(this.getHeight()),
			column: ob1Number1,
		});
	}

	private writeToCursor(cursor: Cursor, char: Column) {
		this.fillCursor(cursor);

		const line = this.getLine(cursor.line);
		const colIndex = ob1Get1(cursor.column) - 1;

		const existing = line.columns[colIndex];
		if (existing !== undefined && existing !== "") {
			//throw new Error(pretty`Trying to write ${char} but already populated with ${existing}. Debug state: ${this.debugState()}`);
		}

		line.columns[colIndex] = char;

		if (cursor.column > this.width) {
			this.width = cursor.column;
		}
	}

	// Build up columns with as many tabs as we can and then spaces
	private getSpaces(count: number): Columns {
		let columns: Columns = [];
		if (count === 0) {
			return columns;
		}

		while (count >= this.tabSize) {
			columns = columns.concat(this.indentColumns);
			count -= this.tabSize;
		}

		while (count > 0) {
			columns.push(" ");
			count--;
		}

		return columns;
	}

	private drawSpaces(count: number) {
		for (const char of this.getSpaces(count)) {
			this.drawChar(char);
		}
	}

	private drawIndent() {
		for (const char of this.indentColumns) {
			this.drawChar(char);
		}
	}

	private userChar(char: Column) {
		if (char === "\t") {
			this.drawIndent();
		} else if (char === "\n") {
			this.userNewline();
		} else {
			this.drawChar(char);
		}
	}

	private drawChar(char: Column) {
		this.writeToCursor(this.cursor, char);
		this.moveCursorRight();
	}

	private drawText(tag: MarkupParsedText, ancestry: Ancestry) {
		this.writeText(tag.value, ancestry, tag.source);

		if (!tag.source && tag.sourceValue !== undefined) {
			for (const char of tag.sourceValue) {
				this.moveSourceCursor(char);
			}
		}
	}

	private moveSourceCursor(char: string) {
		if (char === "\n") {
			this.sourceCursor.currentLineText = "";
			this.sourceCursor.currentColumn = ob1Number1;
			this.sourceCursor.currentLine = ob1Inc(this.sourceCursor.currentLine);
		} else {
			this.sourceCursor.currentLineText += char;
			this.sourceCursor.currentColumn = ob1Inc(this.sourceCursor.currentColumn);
		}
	}

	public writeText(text: string, ancestry: Ancestry, source: boolean) {
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
				this.newline();
			}
			forceNextWordOverflow = false;

			for (const char of splitChars(word)) {
				this.userChar(char);

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
				this.drawChar(" ");
			}
		}

		const end = this.getCursor();
		this.addCursorRange(start, end, ancestry);
	}

	private setRange(
		lineNo: Number1,
		start: Number1,
		end: Number1,
		ancestry: Ancestry,
	) {
		if (start === end) {
			// Nothing to format. Empty tag.
			return;
		}

		const line = this.getLine(lineNo);

		if (end < start) {
			throw new Error(
				pretty`Range end for line index ${lineNo} is before the start. end(${end}) < start(${start}). Debug state: ${this.debugState()}`,
			);
		}

		const {ranges} = line;

		for (const range of ranges) {
			if (
				(start >= range.start && end <= range.end) ||
				(range.start >= start && range.end <= end)
			) {
				throw new Error(
					pretty`The line no #${lineNo} ranges ${range.start}-${range.end} (${sliceColumns(
						line.columns,
						range.start,
						range.end,
					)}) and ${start}-${end} (${sliceColumns(line.columns, start, end)}) overlap. Debug state: ${this.debugState()}`,
				);
			}
		}

		ranges.push({
			start,
			end,
			ancestry,
		});
	}

	private addCursorRange(start: Cursor, end: Cursor, ancestry: Ancestry) {
		if (ancestry.length === 0) {
			// No point storing a range without ancestry
			return;
		}

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
				ob1Inc(this.getLineWidth(start.line)),
				ancestry,
			);

			// Add middle lines
			for (let i = ob1Get1(start.line) + 1; i < ob1Get1(end.line); i++) {
				const line = ob1Coerce1(i);
				this.setRange(line, ob1Coerce1(1), this.getLineWidth(line), ancestry);
			}

			// Add last line
			this.setRange(end.line, ob1Coerce1(1), end.column, ancestry);
		}
	}

	private drawListTag(tag: MarkupParsedTag, ancestry: Ancestry) {
		let items: Array<MarkupParsedTag> = [];
		for (const child of tag.children) {
			if (child.type === "Tag" && child.name === "li") {
				items.push(child);
			}
		}
		if (items.length === 0) {
			return;
		}

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
				this.drawSpaces(highestNumSize - humanNum.length);
				this.writeText(
					`${humanNum}. `,
					[createTag("dim", createEmptyAttributes())],
					false,
				);
				this.drawViewTag(item, ancestry);
				this.newline();
			}
		} else {
			for (const item of items) {
				this.writeText("- ", [createTag("dim", createEmptyAttributes())], false);
				this.drawViewTag(item, ancestry);
				this.newline();
			}
		}
	}

	private drawPointer(): boolean {
		const {pointer} = this.options.view;
		const {sourceCursor, lineStartMeta, cursor} = this;
		if (pointer === undefined) {
			return false;
		}

		if (sourceCursor.currentLine !== pointer.line) {
			// I'm not quite sure what we are meant to do here
			return false;
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
				markerSize += this.tabSize - 1;
			}
		}

		markerSize = Math.max(1, markerSize);

		// If any previous text on this line contains tabs then increase the offset
		for (let i = 0; i < start; i++) {
			if (sourceCursor.currentLineText[i] === "\t") {
				markerOffset += this.tabSize - 1;
			}
		}

		this.newline();

		// Pointer offset
		//this.drawSpaces(markerOffset);
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
			this.drawViewTag(createTag("view", undefined, pointer.message), []);
		}

		return true;
	}

	public parse(
		sub: undefined | string | AnyMarkup,
		offsetPosition: undefined | Position,
	): MarkupParsedChildren {
		if (sub === undefined) {
			return [];
		}

		return this.normalizeChildren(
			parseMarkup(sub, {offsetPosition, sourceText: this.options.sourceText}),
		);
	}

	private getViewLinePrefixes(
		children: Array<MarkupParsedTag>,
		ancestry: Ancestry,
	): {
		width: Number1;
		pointer: undefined | Grid;
		first: undefined | Grid;
		middle: undefined | Grid;
		last: undefined | Grid;
	} {
		const prefixes: Array<MarkupParsedTag> = [];

		let linePrefixFirst: undefined | MarkupParsedTag;
		let linePrefixMiddle: undefined | MarkupParsedTag;
		let linePrefixLast: undefined | MarkupParsedTag;
		let linePrefixPointer: undefined | MarkupParsedTag;

		// Extract viewLinePrefix tags
		for (const child of children) {
			switch (child.attributes.get("type").asStringOrVoid()) {
				case "first": {
					linePrefixFirst = child;
					prefixes.push(linePrefixFirst);
					break;
				}

				case "middle": {
					linePrefixMiddle = child;
					if (linePrefixLast === undefined) {
						linePrefixLast = child;
					}
					prefixes.push(child);
					break;
				}

				case "end": {
					linePrefixLast = child;
					prefixes.push(child);
					break;
				}

				case "pointer": {
					linePrefixPointer = child;
					prefixes.push(child);
					break;
				}

				case undefined: {
					if (
						linePrefixPointer === undefined ||
						linePrefixFirst === undefined ||
						linePrefixMiddle === undefined ||
						linePrefixLast === undefined
					) {
						prefixes.push(child);
					}
					if (linePrefixFirst === undefined) {
						linePrefixFirst = child;
					}
					if (linePrefixMiddle === undefined) {
						linePrefixMiddle = child;
					}
					if (linePrefixLast === undefined) {
						linePrefixLast = child;
					}
					if (linePrefixPointer === undefined) {
						linePrefixPointer = child;
					}
					break;
				}
			}
		}

		const childrenToGrid: Map<undefined | MarkupParsedTag, Grid> = new Map();
		let maxWidth = ob1Coerce1(0);

		// Get the maxWidth
		for (const prefix of prefixes) {
			const grid = new Grid({...this.options, columns: undefined});
			grid.drawChildren(prefix.children, ancestry);
			const width = grid.getSize().width;
			if (width > maxWidth) {
				maxWidth = width;
			}
		}

		// Now actually render the grids
		for (const prefix of prefixes) {
			const grid = new Grid({...this.options, columns: maxWidth});
			grid.drawChildren(prefix.children, ancestry);
			grid.maybeAlign(prefix);
			childrenToGrid.set(prefix, grid);
		}

		return {
			width: maxWidth,
			pointer: childrenToGrid.get(linePrefixPointer),
			first: childrenToGrid.get(linePrefixFirst),
			middle: childrenToGrid.get(linePrefixMiddle),
			last: childrenToGrid.get(linePrefixLast),
		};
	}

	private getViewPointer({attributes, children}: MarkupParsedTag): GridPointer {
		return {
			char: this.parse(
				attributes.get("char").asString(""),
				attributes.get("char").getDiagnosticLocation("inner-value").start,
			),
			message: children,
			line: attributes.get("line").asOneIndexedNumber(0),
			columnStart: attributes.get("start").asOneIndexedNumber(0),
			columnEnd: attributes.get("end").asOneIndexedNumber(0),
		};
	}

	private drawViewTag(
		tag: MarkupParsedTag,
		ancestry: Ancestry,
		shrinkViewport?: Number1,
	) {
		const tags = extractViewTags(tag);
		const {children} = tags;
		const {attributes} = tag;

		const pointer =
			tags.pointer === undefined ? undefined : this.getViewPointer(tags.pointer);
		const linePrefixes = this.getViewLinePrefixes(tags.linePrefixes, ancestry);
		const startCursor = this.getCursor();

		// Calculate size of view
		let subViewport: undefined | Number1 = undefined;
		const {viewportWidth} = this;
		if (viewportWidth !== undefined) {
			subViewport = viewportWidth;
			subViewport = ob1Sub(subViewport, startCursor.column);
			// We add on one because we can place a character on the cursor position
			subViewport = ob1Add(subViewport, 1);
			subViewport = ob1Sub(subViewport, linePrefixes.width);
			if (shrinkViewport !== undefined) {
				subViewport = ob1Sub(subViewport, shrinkViewport);
			}
		}

		const lineWrapMode = lineWrapValidator(
			attributes.get("lineWrap").asStringOrVoid(),
		);

		// Bail and just render the children if this view is redundant
		// This can happen since we wrap some other elements in views
		// The following conditions need to be met:
		// - No custom line wrap mode
		// - No pointer
		// - View viewport is the same
		// - No line prefixes
		// - Cursor on the first column
		if (
			lineWrapMode === undefined &&
			pointer === undefined &&
			subViewport !== undefined &&
			subViewport === this.viewportWidth &&
			linePrefixes.width === ob1Coerce1(0) &&
			startCursor.column === ob1Number1
		) {
			this.drawChildren(children, ancestry);
			return;
		}

		// Render first prefix
		if (linePrefixes.first !== undefined) {
			this.drawGrid(linePrefixes.first);
		}

		const grid = new Grid({
			...this.options,
			view: {
				extraSoftWrapIndent: attributes.get("extraSoftWrapIndent").asNumberOrVoid(),
				pointer,
				lineWrapMode,
			},
			columns: subViewport,
		});
		for (const child of children) {
			grid.drawChild(child, ancestry);
		}
		const drewPointer = grid.drawPointer();
		grid.maybeAlign(tag);

		this.moveCursor({
			...startCursor,
			column: ob1Add(startCursor.column, linePrefixes.width),
		});
		this.drawGrid(grid);

		// Add line prefixes if we wrapped
		const height = ob1Get1(grid.getHeight());
		for (let i = 1; i < height; i++) {
			let linePrefix = linePrefixes.middle;

			if (i === height - 1) {
				if (drewPointer) {
					// Don't prefix a pointer
					linePrefix = linePrefixes.pointer;
				} else {
					linePrefix = linePrefixes.last;
				}
			}

			// Correct last offset if we drew a pointer
			if (drewPointer && i === height - 2) {
				linePrefix = linePrefixes.last;
			}

			if (linePrefix !== undefined) {
				this.moveCursor({
					line: ob1Add(startCursor.line, i),
					column: startCursor.column,
				});

				this.drawGrid(linePrefix);
			}
		}

		this.newline();
	}

	private drawTableTag(tag: MarkupParsedTag, ancestry: Ancestry) {
		const rows: Rows = [];

		for (const child of tag.children) {
			if (child.type === "Tag" && child.name === "tr") {
				const row: Array<MarkupParsedTag> = [];

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
					const grid = new Grid({...this.options, columns: undefined});
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

				const grid = new Grid({...this.options, columns: width});
				grid.drawTag(field, ancestry);
				grid.maybeAlign(field);

				this.drawGrid(grid);
				this.moveCursorRight(ob1Inc(width));
			}

			this.newline();
		}
	}

	private drawIndentTag(tag: MarkupParsedTag, ancestry: Ancestry) {
		// Optimization for nested indents
		let levels = 1;
		let children: MarkupParsedChildren = tag.children;
		while (
			children.length === 1 &&
			children[0].type === "Tag" &&
			children[0].name === "indent"
		) {
			children = children[0].children;
			levels++;
		}

		for (let i = 0; i < levels; i++) {
			this.drawIndent();
		}

		this.drawViewTag(
			createTag("view", createEmptyAttributes(), children),
			ancestry,
			ob1Coerce1(levels * 2),
		);
	}

	// Sometimes we derive a Grid from a tag that accepts an align attribute
	private maybeAlign(tag: MarkupParsedTag) {
		if (tag.attributes.get("align").asStringOrVoid() === "right") {
			this.alignRight();
		}
	}

	private drawGrid(grid: Grid) {
		const lines = grid.getTrimmedLines();
		const cursor = this.getCursor();

		for (let lineIndex = 0; lineIndex < lines.length; lineIndex++) {
			const {columns, ranges} = lines[lineIndex];

			const correctLine = ob1Add(cursor.line, lineIndex);

			for (let colIndex = 0; colIndex < columns.length; colIndex++) {
				const char = columns[colIndex];

				this.writeToCursor(
					{
						line: correctLine,
						column: ob1Add(cursor.column, colIndex),
					},
					char,
				);
			}

			for (const range of ranges) {
				this.setRange(
					correctLine,
					ob1Dec(ob1Add(cursor.column, range.start)),
					ob1Dec(ob1Add(cursor.column, range.end)),
					range.ancestry,
				);
			}
		}
	}

	private drawTag(tag: MarkupParsedTag, ancestry: Ancestry) {
		const hook = hooks.get(tag.name);

		const subAncestry: Ancestry = [...ancestry, tag];

		if (hook !== undefined && hook.before !== undefined) {
			hook.before(tag, this, ancestry);
		}

		switch (tag.name) {
			case "ol":
			case "ul": {
				this.drawListTag(tag, subAncestry);
				break;
			}

			case "table": {
				this.drawTableTag(tag, subAncestry);
				break;
			}

			case "view": {
				this.drawViewTag(tag, subAncestry);
				break;
			}

			case "indent": {
				this.drawIndentTag(tag, subAncestry);
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
	}

	private drawChild(child: MarkupParsedChild, ancestry: Ancestry) {
		if (child.type === "Text") {
			this.drawText(child, ancestry);
		} else {
			this.drawTag(child, ancestry);
		}
	}

	public drawChildren(children: MarkupParsedChildren, ancestry: Ancestry) {
		for (const child of children) {
			this.drawChild(child, ancestry);
		}
	}

	private normalizeChildren(
		children: MarkupParsedChildren,
	): MarkupParsedChildren {
		let newChildren: MarkupParsedChildren = [];

		for (const child of children) {
			newChildren = newChildren.concat(this.normalizeChild(child));
		}

		return newChildren;
	}

	private normalizeChild(child: MarkupParsedChild): MarkupParsedChildren {
		if (child.type === "Text") {
			let {value} = child;

			if (this.options.convertTabs && value.includes("\t")) {
				const splitTabs = value.split("\t");
				const children: MarkupParsedChildren = [];

				for (let i = 0; i < splitTabs.length; i++) {
					if (i > 0) {
						children.push({
							type: "Text",
							source: false,
							sourceValue: "\t",
							value: " ".repeat(this.tabSize),
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
				const paddingTextNode: MarkupParsedText = {
					type: "Text",
					source: false,
					value: joinColumns(this.getSpaces(paddingSize)),
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
								value: buildFileLink(tag.attributes, this.options).text,
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

function getChildrenTextLength(children: MarkupParsedChildren): number {
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
		before?: (tag: MarkupParsedTag, grid: Grid, ancestry: Ancestry) => void;
		after?: (tag: MarkupParsedTag, grid: Grid, ancestry: Ancestry) => void;
	}
> = new Map();

hooks.set(
	"hr",
	{
		after: (tag, grid, ancestry) => {
			let viewportWidth =
				grid.viewportWidth === undefined ? 100 : ob1Get1(grid.viewportWidth);
			let size = viewportWidth - ob1Get1(grid.cursor.column) + 1;
			size = Math.max(size, 0);
			grid.writeText("\u2501".repeat(size), ancestry, false);
		},
	},
);
