/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
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
	Markup,
} from "@internal/markup";
import {
	GridLocators,
	GridOptions,
	GridOutputFormat,
	GridPointer,
} from "./types";
import {
	Duration,
	OneIndexed,
	humanizeFileSize,
	humanizeNumber,
} from "@internal/numbers";
import {splitChars} from "@internal/string-utils";

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
	line: OneIndexed;
	column: OneIndexed;
};

type Rows = (MarkupParsedTag[])[];

type Ancestry = MarkupParsedTag[];

type Column = string | symbol;

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

function joinColumns(columns: Column[]): string {
	return columns.filter((column) => typeof column === "string").join("");
}

function sliceColumns(
	columns: Column[],
	start: OneIndexed,
	end: OneIndexed,
): string {
	return joinColumns(columns.slice(start.valueOf() - 1, end.valueOf() - 1));
}

function extractViewTags(
	tag: MarkupParsedTag,
): {
	pointer: undefined | MarkupParsedTag;
	linePrefixes: MarkupParsedTag[];
	children: MarkupParsedChildren;
} {
	let pointer: undefined | MarkupParsedTag;
	let linePrefixes: MarkupParsedTag[] = [];
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
	ranges: {
		start: OneIndexed;
		end: OneIndexed;
		ancestry: Ancestry;
	}[];
	columns: Column[];
};

const TAB_PLACEHOLDER_COLUMN = Symbol("TAB_LAYOUT_PLACEHOLDER");

function isWhitespace(char: Column): boolean {
	return char === " " || char === "\t" || char === TAB_PLACEHOLDER_COLUMN;
}

class GridAddedNewline {}

export default class Grid {
	constructor(opts: GridOptions) {
		this.viewportWidth = opts.columns === undefined ? undefined : opts.columns;
		this.options = opts;

		this.features =
			opts.features === undefined ? DEFAULT_TERMINAL_FEATURES : opts.features;

		this.cursor = {
			line: new OneIndexed(),
			column: new OneIndexed(),
		};

		this.sourceCursor = {
			currentLineText: "",
			currentLine: new OneIndexed(),
			currentColumn: new OneIndexed(),
		};

		this.lineStartMeta = {
			softWrapped: false,
			indentationCount: 0,
			sourceColumn: new OneIndexed(),
		};

		this.locators = new Map();

		const {lineWrapMode = "word-break"} = opts.view;
		this.lineWrapMode = lineWrapMode;
		this.width = new OneIndexed();

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

		const indentColumns: Column[] = [];
		for (let i = 0; i < this.tabSize; i++) {
			let char: Column = " ";
			if (!this.options.convertTabs) {
				char = i === 0 ? "\t" : TAB_PLACEHOLDER_COLUMN;
			}
			indentColumns.push(char);
		}
		this.indentColumns = indentColumns;
	}

	private indentColumns: Column[];
	private tabSize: number;

	public locators: GridLocators;
	public features: TerminalFeatures;
	private lineWrapMode: MarkupLineWrapMode;
	public options: GridOptions;
	private lines: GridLine[];
	public cursor: Cursor;
	private width: OneIndexed;
	public viewportWidth: undefined | OneIndexed;

	// This tracks information about how much of the original source we have printed
	// We use this to point to and highlight certain sections. ie. drawPointer
	private sourceCursor: {
		currentLineText: string;
		currentLine: OneIndexed;
		currentColumn: OneIndexed;
	};

	private lineStartMeta: {
		softWrapped: boolean;
		indentationCount: number;
		sourceColumn: OneIndexed;
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
		const {viewportWidth} = this;
		if (viewportWidth === undefined) {
			return;
		}

		this.lines = this.lines.map(({ranges, columns}) => {
			let newColumns = [];

			// Pad out line to viewport width
			const offset = Math.max(0, viewportWidth.valueOf() - columns.length);
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
					start: range.start.add(offset),
					end: range.end.add(offset),
					ancestry: range.ancestry,
				};
			});

			return {
				ranges: newRanges,
				columns: newColumns,
			};
		});

		// TODO Update locators
	}

	private doesOverflowViewport(column: OneIndexed): boolean {
		return (
			this.lineWrapMode !== "none" &&
			this.viewportWidth !== undefined &&
			column.valueOf() > this.viewportWidth.valueOf()
		);
	}

	private getHeight(): OneIndexed {
		return new OneIndexed(this.lines.length);
	}

	private getLine(line: OneIndexed): GridLine {
		const lineIndex = line.valueOf() - 1;
		for (let i = lineIndex; i >= 0 && this.lines[i] === undefined; i--) {
			this.clearLine(new OneIndexed(i + 1));
		}
		return this.lines[lineIndex];
	}

	private clearLine(line: OneIndexed) {
		this.lines[line.valueOf() - 1] = {ranges: [], columns: []};
	}

	private getLineWidth(line: OneIndexed): OneIndexed {
		return new OneIndexed(this.getLine(line).columns.length);
	}

	public getWidth(): OneIndexed {
		return this.width;
	}

	private getSize(): {
		width: OneIndexed;
		height: OneIndexed;
	} {
		return {
			height: this.getHeight(),
			width: this.getWidth(),
		};
	}

	private getCursor(): Cursor {
		return {...this.cursor};
	}

	public getLines(format: GridOutputFormat): string[] {
		switch (format) {
			case "ansi":
				return this.getFormattedAnsiLines();

			case "html":
				return this.getFormattedHtmlLines();

			case "none":
				return this.getUnformattedLines();
		}
	}

	private getTrimmedLines(): GridLine[] {
		const lines = [...this.lines];

		// Remove empty columns
		// Explicit newlines will have at least one column with an empty field
		while (lines.length > 0 && lines[lines.length - 1].columns.length === 0) {
			lines.pop();
		}

		return lines;
	}

	private getUnformattedLines(): string[] {
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
	): string[] {
		const lines: string[] = [];

		for (const {ranges, columns} of this.getTrimmedLines()) {
			// Sort ranges from last to first
			const sortedRanges = ranges.sort((a, b) =>
				b.end.valueOf() - a.end.valueOf()
			);

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
				catchUp(end.valueOf() - 1);

				let substr = opts.normalizeText(sliceColumns(columns, start, end));

				// Format tags in reverse
				for (let i = ancestry.length - 1; i >= 0; i--) {
					const tag = ancestry[i];
					substr = opts.formatTag(tag, substr);
				}

				substr = opts.wrapRange(substr);

				line = `${substr}${line}`;
				lastEnd = start.valueOf() - 1;
			}

			catchUp(0);

			lines.push(line.trimRight());
		}

		return lines;
	}

	private getFormattedHtmlLines(): string[] {
		return this.getFormattedLines({
			normalizeText: (text) => escapeXHTMLEntities(text),
			formatTag: (tag, inner) => htmlFormatText(tag, inner),
			wrapRange: (str) => str,
		});
	}

	private getFormattedAnsiLines(): string[] {
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
		const colIndex = cursor.column.valueOf() - 1;

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
			if (this.options.throwOnNewline) {
				throw new GridAddedNewline();
			}

			this.lineStartMeta.softWrapped = false;
			this.lineStartMeta.indentationCount = 0;
			this.lineStartMeta.sourceColumn = this.sourceCursor.currentColumn;
		}

		this.cursor = cursor;
	}

	private moveCursorRight(columns: OneIndexed = new OneIndexed()) {
		const newColumns = this.cursor.column.add(columns);

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
			line: this.getHeight().increment(),
			column: new OneIndexed(),
		});
	}

	private writeToCursor(cursor: Cursor, char: Column) {
		this.fillCursor(cursor);

		const line = this.getLine(cursor.line);
		const colIndex = cursor.column.valueOf() - 1;

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
	private getSpaces(count: number): Column[] {
		let columns: Column[] = [];
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
			this.sourceCursor.currentColumn = new OneIndexed();
			this.sourceCursor.currentLine = this.sourceCursor.currentLine.increment();
		} else {
			this.sourceCursor.currentLineText += char;
			this.sourceCursor.currentColumn = this.sourceCursor.currentColumn.increment();
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
				(this.doesOverflowViewport(this.cursor.column.add(word.length - 1)) &&
				!this.doesOverflowViewport(new OneIndexed(word.length))));
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
				this.cursor.column.valueOf() === 1 &&
				word !== ""
			) {
				ignoreTrailingSpace = true;
			}

			// If the next word will cause an overflow then don't print a leading space as it will be pointless
			const nextWord = words[i + 1];
			if (
				this.lineWrapMode === "word-break" &&
				nextWord !== undefined &&
				this.doesOverflowViewport(this.cursor.column.add(nextWord.length))
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
		lineNo: OneIndexed,
		start: OneIndexed,
		end: OneIndexed,
		ancestry: Ancestry,
	) {
		if (start.equal(end)) {
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

		if (start.line.equal(end.line)) {
			if (start.column.equal(end.column)) {
				// Empty range
				return;
			}

			this.setRange(start.line, start.column, end.column, ancestry);
		} else {
			// Add first line
			this.setRange(
				start.line,
				start.column,
				this.getLineWidth(start.line).increment(),
				ancestry,
			);

			// Add middle lines
			for (let i = start.line.valueOf() + 1; i < end.line.valueOf(); i++) {
				const line = new OneIndexed(i);
				const start = new OneIndexed();
				const end = this.getLineWidth(line).increment();
				this.setRange(line, start, end, ancestry);
			}

			// Add last line
			this.setRange(end.line, new OneIndexed(), end.column, ancestry);
		}
	}

	private drawLocatorTag(tag: MarkupParsedTag, ancestry: Ancestry) {
		const id = tag.attributes.get("id").asString("default");
		const start = cursorToPosition(this.getCursor());
		this.drawChildren(tag.children, ancestry);
		const end = cursorToPosition(this.getCursor());
		this.locators.set(id, {start, end});
	}

	private drawListTag(tag: MarkupParsedTag, ancestry: Ancestry) {
		let items: MarkupParsedTag[] = [];
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
			const joinSameLine = tag.attributes.get("joinSameLine").asStringOrVoid();
			if (joinSameLine === undefined) {
				for (const item of items) {
					this.writeText(
						"- ",
						[createTag("dim", createEmptyAttributes())],
						false,
					);
					this.drawViewTag(item, ancestry);
					this.newline();
				}
			} else {
				this.drawULJoinSameLine(joinSameLine, items, ancestry);
			}
		}
	}

	private drawULJoinSameLine(
		joinSameLine: string,
		items: MarkupParsedTag[],
		ancestry: Ancestry,
	) {
		// If we have no grid size then we default to 500. We do this to prevent drawing super long lines when the intent
		// of this attribute is to reduce excessive newlines.
		const viewportWidth = this.viewportWidth ?? new OneIndexed(150);
		const subViewport = viewportWidth.subtract(this.cursor.column);

		const grid = new Grid({
			...this.options,
			columns: subViewport,
			throwOnNewline: true,
		});

		try {
			// Draw items separated by the provided attribute
			for (let i = 0; i < items.length; i++) {
				const item = items[i];
				grid.drawTag(item, ancestry);
				if (i !== items.length - 1) {
					grid.drawText(
						{
							type: "Text",
							source: false,
							value: joinSameLine,
						},
						ancestry,
					);
				}
			}

			// Didn't error so we must be able to fit on the same line!
			this.drawGrid(grid);
			this.moveCursor({
				line: this.cursor.line,
				column: this.cursor.column.add(grid.width),
			});
		} catch (err) {
			if (err instanceof GridAddedNewline) {
				// Cannot fit on a newline so indent and draw the rest of the items
				for (const item of items) {
					this.newline();
					this.drawIndent();
					this.drawViewTag(item, ancestry);
				}
				this.newline();
			} else {
				throw err;
			}
		}
	}

	private drawPointer(): boolean {
		const {pointer} = this.options.view;
		const {sourceCursor, lineStartMeta, cursor} = this;
		if (pointer === undefined) {
			return false;
		}

		if (!sourceCursor.currentLine.equal(pointer.line)) {
			// I'm not quite sure what we are meant to do here
			return false;
		}

		let start = pointer.columnStart.valueOf();
		let end = pointer.columnEnd.valueOf();

		if (!cursor.line.equal(sourceCursor.currentLine)) {
			start = 0;
			end = end - lineStartMeta.sourceColumn.valueOf();
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
		sub: undefined | Markup,
		offsetPosition: undefined | Position,
		cache: boolean,
	): MarkupParsedChildren {
		if (sub === undefined) {
			return [];
		}

		return this.normalizeChildren(
			parseMarkup(sub, {offsetPosition, sourceText: this.options.sourceText}, cache),
		);
	}

	private getViewLinePrefixes(
		children: MarkupParsedTag[],
		ancestry: Ancestry,
	): {
		width: OneIndexed;
		pointer: undefined | Grid;
		first: undefined | Grid;
		middle: undefined | Grid;
		last: undefined | Grid;
	} {
		const prefixes: MarkupParsedTag[] = [];

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
		let maxWidth = new OneIndexed(0);

		// Get the maxWidth
		for (const prefix of prefixes) {
			const grid = new Grid({...this.options, columns: undefined});
			grid.drawChildren(prefix.children, ancestry);
			const width = grid.getSize().width;
			if (width.valueOf() > maxWidth.valueOf()) {
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
				false,
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
		shrinkViewport?: OneIndexed,
	) {
		const tags = extractViewTags(tag);
		const {children} = tags;
		const {attributes} = tag;

		const pointer =
			tags.pointer === undefined ? undefined : this.getViewPointer(tags.pointer);
		const linePrefixes = this.getViewLinePrefixes(tags.linePrefixes, ancestry);
		const startCursor = this.getCursor();

		// Calculate size of view
		let subViewport: undefined | OneIndexed = undefined;
		const {viewportWidth} = this;
		if (viewportWidth !== undefined) {
			subViewport = viewportWidth;
			subViewport = subViewport.subtract(startCursor.column);
			// We add on one because we can place a character on the cursor position
			subViewport = subViewport.add(1);
			subViewport = subViewport.subtract(linePrefixes.width);
			if (shrinkViewport !== undefined) {
				subViewport = subViewport.subtract(shrinkViewport);
			}
		}

		const lineWrapMode = lineWrapValidator(
			attributes.get("lineWrap").asStringOrVoid(),
		);

		// Bail and just render the children if this view is redundant
		// This can happen since we wrap some other elements in views
		// The following conditions need to be met:
		// - No custom line wrap mode
		// - No align attribute
		// - No pointer
		// - View viewport is the same
		// - No line prefixes
		// - Cursor on the first column
		if (
			lineWrapMode === undefined &&
			pointer === undefined &&
			subViewport !== undefined &&
			subViewport.equal(this.viewportWidth) &&
			!attributes.has("align") &&
			linePrefixes.width.valueOf() === 0 &&
			startCursor.column.valueOf() === 1
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
			column: startCursor.column.add(linePrefixes.width),
		});
		this.drawGrid(grid);

		// Add line prefixes if we wrapped
		const height = grid.getHeight().valueOf();
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
					line: startCursor.line.add(i),
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
				const row: MarkupParsedTag[] = [];

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
		const columnWidths: number[] = [];
		for (let i = 0; i < columnCount; i++) {
			const widths = rows.map((row): number => {
				const field = row[i];
				if (field === undefined) {
					// Could be an excessive column
					return 0;
				} else {
					const grid = new Grid({...this.options, columns: undefined});
					grid.drawTag(field, ancestry);
					return grid.getSize().width.valueOf();
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
				: viewportWidth.valueOf() - columnCount - 1;
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
				const width = new OneIndexed(columnWidths[colIndex]);

				const grid = new Grid({...this.options, columns: width});
				grid.drawTag(field, ancestry);
				grid.maybeAlign(field);

				this.drawGrid(grid);
				this.moveCursorRight(width.increment());
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
			// We also indent the right side
			new OneIndexed(levels * 2),
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

		for (const [key, locator] of grid.locators) {
			this.locators.set(
				key,
				{
					start: {
						line: cursor.line.add(locator.start.line),
						column: locator.start.column.add(cursor.column.toZeroIndexed()),
					},
					end: {
						line: cursor.line.add(locator.end.line),
						column: locator.end.column.add(cursor.column.toZeroIndexed()),
					},
				},
			);
		}

		for (let lineIndex = 0; lineIndex < lines.length; lineIndex++) {
			const {columns, ranges} = lines[lineIndex];

			const correctLine = cursor.line.add(lineIndex);

			for (let colIndex = 0; colIndex < columns.length; colIndex++) {
				const char = columns[colIndex];

				this.writeToCursor(
					{
						line: correctLine,
						column: cursor.column.add(colIndex),
					},
					char,
				);
			}

			for (const range of ranges) {
				this.setRange(
					correctLine,
					cursor.column.add(range.start).decrement(),
					cursor.column.add(range.end).decrement(),
					range.ancestry,
				);
			}
		}
	}

	private drawTag(tag: MarkupParsedTag, ancestry: Ancestry) {
		const hook = hooks.get(tag.name);

		const subAncestry: Ancestry = [...ancestry, tag];

		if (hook?.before !== undefined) {
			hook.before(tag, this, ancestry);
		}

		switch (tag.name) {
			case "locator": {
				this.drawLocatorTag(tag, subAncestry);
				break;
			}

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

		if (hook?.after !== undefined) {
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
										Duration.fromNanoseconds(BigInt(singleInnerText)).format(),
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
				grid.viewportWidth === undefined ? 100 : grid.viewportWidth.valueOf();
			let size = viewportWidth - grid.cursor.column.valueOf() + 1;
			size = Math.max(size, 0);
			grid.writeText("\u2501".repeat(size), ancestry, false);
		},
	},
);

function cursorToPosition(cursor: Cursor): Position {
	return {
		line: cursor.line,
		column: cursor.column.toZeroIndexed(),
	};
}
