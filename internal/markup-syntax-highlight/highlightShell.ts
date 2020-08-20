import {HighlightCodeResult} from "./types";
import {concatMarkup, filePathToMarkup, markup} from "@internal/markup";
import {concatSplitLinesMarkup, markupToken} from "./utils";
import {createUnknownPath} from "@internal/path";
import {markupTag} from "@internal/markup/escape";

// Very crude. Should be updated to support hash bangs and other fancy syntax
// Right now we just need it to highlight the CLI snippets we output
// Ref: https://github.com/PrismJS/prism/blob/master/components/prism-bash.js

export default function highlightShell(
	opts: {
		input: string;
		isShorthand?: boolean;
	},
): HighlightCodeResult {
	// TODO properly handle strings with spaces
	const segments: Array<string> = opts.input.split(" ");

	let segmentCount = 0;

	const parts = segments.map((segment, i) => {
		if (i === 0) {
			const lastChar = segment[segment.length - 1];
			if (lastChar === "#" || lastChar === "$") {
				// const punc = markupToken("punctuation", lastChar);
				const punc = markupTag("emphasis", markup`${lastChar}`);
				if (segment.length === 1) {
					return punc;
				} else {
					return concatMarkup([
						filePathToMarkup(createUnknownPath(segment.slice(0, -1)), true),
						punc,
					]);
				}
			}
		}

		if (segment[0] === '"' || segment[0] === "'") {
			return markupToken("string", segment);
		}

		if (segment[0] === "-") {
			//return markupToken("operator", segment);
		}

		segmentCount++;

		if (segmentCount === 1 || (opts.isShorthand && segmentCount === 2)) {
			return markupToken("function", segment);
		}

		return markup`<dim>${segment}</dim>`;
	});

	return concatSplitLinesMarkup([concatMarkup(parts, markup` `)]);
}
