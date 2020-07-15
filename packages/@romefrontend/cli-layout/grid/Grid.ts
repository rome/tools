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
	TagAttributes,
	TagNode,
	TextNode,
} from "../types";
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
} from "@romefrontend/cli-environment";
import {parseMarkup} from "../parse";
import {Position} from "@romefrontend/parser-core";
import {lineWrapValidator} from "../tags";
import {formatAnsi} from "../ansi";

type Cursor = {
	line: Number1;
	column: Number1;
};

type Rows = Array<Array<TagNode>>;

type Ancestry = Array<TagNode>;

function createTag(
	name: TagNode["name"],
	attributes: TagNode["attributes"] = createEmptyAttributes(),
	children: TagNode["children"] = [],
): TagNode {
	return {
		type: "Tag",
		name,
		attributes,
		children,
	};
}

function extractViewTags(
	tag: TagNode,
): {
	pointer: undefined | TagNode;
	linePrefixes: Array<TagNode>;
	children: Children;
} {
	let pointer: undefined | TagNode;
	let linePrefixes: Array<TagNode> = [];
	let children: Children = [];

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
	columns: Array<string>;
};

export default class Grid {
	constructor(opts: MarkupFormatGridOptions) {
		this.viewportWidth = opts.columns === undefined ? undefined : opts.columns;
		this.options = opts;

		this.features =
			opts.features === undefined ? TERMINAL_FEATURES_DEFAULT : opts.features;

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

		this.lines = [];
	}

	features: TerminalFeatures;
	lineWrapMode: MarkupLineWrapMode;
	options: MarkupFormatGridOptions;
	lines: Array<GridLine>;
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
		softWrapped: boolean;
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
			let offset = 0;

			// Pad out line to viewport width
			while (newColumns.length < viewportWidth) {
				offset++;
				newColumns.unshift(" ");
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

	getLine(line: Number1): GridLine {
		const lineIndex = ob1Get1(line) - 1;
		for (let i = lineIndex; i >= 0 && this.lines[i] === undefined; i--) {
			this.clearLine(ob1Coerce1(i + 1));
		}
		return this.lines[lineIndex];
	}

	clearLine(line: Number1) {
		this.lines[ob1Get1(line) - 1] = {ranges: [], columns: []};
	}

	getLineWidth(line: Number1): Number1 {
		return ob1Coerce1(this.getLine(line).columns.length);
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

	getTrimmedLines(): Array<GridLine> {
		const lines = [...this.lines];

		// Remove empty columns
		// Explicit newlines will have at least one column with an empty field
		while (lines.length > 0 && lines[lines.length - 1].columns.length === 0) {
			lines.pop();
		}

		return lines;
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
				line = `${columns.slice(end, lastEnd).join("")}${line}`;
				lastEnd = end;
			}

			for (const {start, end, ancestry} of sortedRanges) {
				const startIndex = ob1Get1(start) - 1;
				const endIndex = ob1Get1(end) - 1;
				catchUp(endIndex);

				let substr = opts.normalizeText(
					columns.slice(startIndex, endIndex).join(""),
				);

				// Format tags in reverse
				for (let i = ancestry.length - 1; i >= 0; i--) {
					const tag = ancestry[i];
					substr = opts.formatTag(tag, substr);
				}

				substr = opts.wrapRange(substr);

				line = `${substr}${line}`;
				lastEnd = startIndex;
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
			wrapRange: (str) => str,
		});
	}

	getFormattedAnsiLines(): Array<string> {
		return this.getFormattedLines({
			normalizeText: (text) => text,
			formatTag: (tag, inner) => {
				return ansiFormatText(tag, inner, this);
			},
			wrapRange: (str) => formatAnsi.reset(str),
		});
	}

	fillCursor(cursor: Cursor) {
		const line = this.getLine(cursor.line);

		const colIndex = ob1Get1(cursor.column) - 1;
		for (let i = colIndex - 1; i >= 0 && line.columns[i] === undefined; i--) {
			line.columns[i] = " ";
		}
	}

	moveCursor(cursor: Cursor) {
		if (cursor.line !== this.cursor.line) {
			this.lineStartMeta.softWrapped = false;
			this.lineStartMeta.indentationCount = 0;
			this.lineStartMeta.sourceColumn = this.sourceCursor.currentColumn;
		}

		this.cursor = cursor;
	}

	moveCursorRight(columns: Number1 = ob1Number1) {
		const newColumns = ob1Add(this.cursor.column, columns);

		// Perform character line wrap
		if (this.doesOverflowViewport(newColumns)) {
			const currentLine = this.getLine(this.cursor.line);
			const previousLineSoftWrapped = this.lineStartMeta.softWrapped;

			this.newline();

			// Soft wrap, inherit the previous lines indentation
			if (currentLine !== undefined) {
				for (
					let i = 0;
					i < currentLine.columns.length && currentLine.columns[i] === " ";
					i++
				) {
					this.moveCursorRight();
					this.lineStartMeta.indentationCount++;
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

	userNewline() {
		this.newline();
		this.writeToCursor(this.cursor, "");
	}

	newline() {
		this.moveCursor({
			line: ob1Inc(this.getHeight()),
			column: ob1Number1,
		});
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

		const line = this.getLine(cursor.line);
		const colIndex = ob1Get1(cursor.column) - 1;
		line.columns[colIndex] = char;

		if (cursor.column > this.width) {
			this.width = cursor.column;
		}
	}

	writeChar(char: string) {
		if (char === "\n") {
			this.userNewline();
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
		const {sourceCursor} = this;
		const {pointer} = this.options.view;

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
				this.newline();
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

	setRange(line: Number1, start: Number1, end: Number1, ancestry: Ancestry) {
		if (start === end) {
			// Nothing to format. Empty tag.
			return;
		}

		if (end < start) {
			throw new Error(
				`Range end for line index ${line} is before the start. end(${end}) < start(${start}).\nLine content: ${JSON.stringify(
					this.getLine(line).columns.join(""),
				)}`,
			);
		}

		const {ranges} = this.getLine(line);

		for (const range of ranges) {
			if (
				(start >= range.start && end <= range.end) ||
				(range.start >= start && range.end <= end)
			) {
				throw new Error(
					`The ranges ${range.start}-${range.end} and ${start}-${end} overlap`,
				);
			}
		}

		ranges.push({
			start,
			end,
			ancestry,
		});
	}

	addCursorRange(start: Cursor, end: Cursor, ancestry: Ancestry) {
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

		this.newline();

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
				this.drawView(item, ancestry);
				this.newline();
			}
		} else {
			for (const item of items) {
				this.writeText("- ", [createTag("dim", createEmptyAttributes())], false);
				this.drawView(item, ancestry);
				this.newline();
			}
		}
	}

	drawPointer(): boolean {
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
				markerSize++;
			}
		}

		markerSize = Math.max(1, markerSize);

		// If any previous text on this line contains tabs then increase the offset
		for (let i = 0; i < start; i++) {
			if (sourceCursor.currentLineText[i] === "\t") {
				markerOffset++;
			}
		}

		this.newline();

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
			this.drawView(createTag("view", undefined, pointer.message), []);
		}

		return true;
	}

	parse(sub: undefined | string, offsetPosition: undefined | Position): Children {
		if (sub === undefined) {
			return [];
		}

		return this.normalizeChildren(
			parseMarkup(sub, {offsetPosition, sourceText: this.options.sourceText}),
		);
	}

	parseAttribute(attributes: TagAttributes, key: string): Children {
		return this.parse(
			attributes.get(key).asStringOrVoid(),
			attributes.get(key).getDiagnosticLocation("inner-value").start,
		);
	}

	getViewLinePrefixes(
		children: Array<TagNode>,
		ancestry: Ancestry,
	): {
		width: Number1;
		pointer: undefined | Grid;
		first: undefined | Grid;
		middle: undefined | Grid;
		last: undefined | Grid;
	} {
		const prefixes: Array<TagNode> = [];

		let linePrefixFirst: undefined | TagNode;
		let linePrefixMiddle: undefined | TagNode;
		let linePrefixLast: undefined | TagNode;
		let linePrefixPointer: undefined | TagNode;

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

		const childrenToGrid: Map<undefined | TagNode, Grid> = new Map();
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

	getViewPointer({attributes, children}: TagNode): MarkupPointer {
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

	drawView(tag: TagNode, ancestry: Ancestry, shrinkViewport?: Number1) {
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
		if (
			lineWrapMode === undefined &&
			pointer === undefined &&
			subViewport === this.viewportWidth &&
			linePrefixes.width === ob1Coerce1(0)
		) {
			this.drawChildren(children, ancestry);
			return;
		}

		// Render first prefix
		if (linePrefixes.first !== undefined) {
			this.drawGrid(linePrefixes.first);
		}
		this.moveCursor({
			...startCursor,
			column: ob1Add(startCursor.column, linePrefixes.width),
		});

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
		this.drawGrid(grid);

		// Add on any subsequent line prefixes if we wrapped
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

	drawTable(tag: TagNode, ancestry: Ancestry) {
		const rows: Rows = [];

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

	// Sometimes we derive a Grid from a tag that accepts an align attribute
	maybeAlign(tag: TagNode) {
		if (tag.attributes.get("align").asStringOrVoid() === "right") {
			this.alignRight();
		}
	}

	drawGrid(grid: Grid) {
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
					ob1Coerce1(levels * 2),
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

	drawChild(child: ChildNode, ancestry: Ancestry) {
		if (child.type === "Text") {
			this.drawText(child, ancestry);
		} else {
			this.drawTag(child, ancestry);
		}
	}

	drawChildren(children: Children, ancestry: Ancestry) {
		for (const child of children) {
			this.drawChild(child, ancestry);
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
			let viewportWidth =
				grid.viewportWidth === undefined ? 100 : ob1Get1(grid.viewportWidth);
			let size = viewportWidth - ob1Get1(grid.cursor.column) + 1;
			size = Math.max(size, 0);
			grid.writeText("\u2501".repeat(size), ancestry, false);
		},
	},
);
