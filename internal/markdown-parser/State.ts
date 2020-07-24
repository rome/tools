import {ParserCore} from "@internal/parser-core";
import {MarkdownParserState} from "@internal/markdown-parser/types";

type Pinpoint = {
	index: number;
	type: DelimiterType;
};

type DelimiterType = "Emphasis" | "Strong";

type Delimiter = {
	start: number;
	end: number;
	type: DelimiterType;
};

export class InlineState {
	constructor() {
		this.pinpoints = new Set();
		this.delimiters = new Set();
	}
	private pinpoints: Set<Pinpoint>;
	private delimiters: Set<Delimiter>;

	/**
     *
     * Every time a delimiter run can open an emphasis, gets marked with its type and index
     *
     * @param index
     * @param type
     */
	public registerStartOfDelimiter(index: number, type: DelimiterType) {
		this.pinpoints.add({
			index,
			type,
		});
	}

	/**
     * Check is the OPENING delimiter is connected.
     * If so, it returns the end index.
     *
     * If not connected, returns false.
     * @param end
     * @param possibleType
     */
	public isDelimiterClosed(
		end: number,
		possibleType: DelimiterType,
	): false | number {
		let found = undefined;
		for (const [delimiter] of this.delimiters.entries()) {
			if (possibleType === delimiter.type && delimiter.end === end) {
				found = delimiter.start;
				break;
			}
		}

		if (found !== undefined) {
			return found;
		}

		return false;
	}

	/**
     * Given an index and a type, it connects to the first available pinpoint.
     *
     * @param endIndex
     * @param type
     */
	public connectDelimiter(endIndex: number, type: DelimiterType) {
		if (this.pinpoints.size > 0) {
			// get the most recent pinpoint that that matches the given type
			const pinpoint = Array.from(this.pinpoints).reverse().find((p) =>
				p.type === type
			);
			if (pinpoint) {
				// pinpoint found, make che connection
				this.delimiters.add({
					start: pinpoint.index,
					end: endIndex,
					type,
				});
				// now we have to delete possible old delimiters
				// Given the following string: Lorem _Ipsum is _simply dummy_ text of the_ printing
				// When we connect the underscore in "the_", we still have a delimiter between
				// _simply and dummy_ and we want to remove them because the new one will swallow the old one
				for (const [delimiter] of this.delimiters.entries()) {
					if (delimiter.type === type) {
						if (delimiter.start > pinpoint.index) {
							this.delimiters.delete(delimiter);
							break;
						}
						if (delimiter.end < endIndex) {
							this.delimiters.delete(delimiter);
							break;
						}
					}
				}
				// remove used pinpoint
				this.pinpoints.delete(pinpoint);
			} else {
				// This is for the following case
				// Lorem Ipsum _is simply dummy_ text of the_ printing
				// We finished the pinpoints but the_ can connect with _is,
				// so we remove the old delimiter
				for (const [delimiter] of this.delimiters.entries()) {
					if (delimiter.type === type && delimiter.end < endIndex) {
						delimiter.end = endIndex;
						break;
					}
				}
			}
		}
	}

	public getDelimiters() {
		return this.delimiters;
	}

	public purge() {
		this.pinpoints.clear();
		this.delimiters.clear();
	}
}

export function createMarkdownInitialState(): MarkdownParserState {
	return {
		...ParserCore.createInitialState(),
		isParagraph: false,
		isBlockHead: false,
		inlineState: new InlineState(),
	};
}
