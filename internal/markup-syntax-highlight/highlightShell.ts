import {AnsiHighlightOptions, HighlightCodeResult} from "./types";
import {concatMarkup, filePathToMarkup, markup} from "@internal/markup";
import {concatSplitLinesMarkup, markupToken} from "./utils";
import {createUnknownFilePath} from "@internal/path";

// Very crude. Should be updated to support hash bangs and other fancy syntax
// Right now we just need it to highlight the CLI snippets we output
// Ref: https://github.com/PrismJS/prism/blob/master/components/prism-bash.js

export default function highlightShell(
	opts: AnsiHighlightOptions,
): HighlightCodeResult {
	// TODO properly handle strings with spaces
	const segments: Array<string> = opts.input.split(" ");

	let firstCommandSegment = true;

	const parts = segments.map((segment, i) => {
		if (i === 0) {
			const lastChar = segment[segment.length - 1];
			if (lastChar === "#" || lastChar === "$") {
				return concatMarkup([
					filePathToMarkup(createUnknownFilePath(segment.slice(0, -1)), true),
					markupToken("punctuation", lastChar),
				]);
			}
		}

		if (firstCommandSegment) {
			firstCommandSegment = false;
			return markupToken("function", segment);
		}

		if (segment[0] === '"' || segment[0] === "'") {
			return markupToken("string", segment);
		}

		if (segment[0] === "-") {
			return markupToken("operator", segment);
		}

		return markup`${segment}`;
	});

	return concatSplitLinesMarkup([concatMarkup(parts, markup` `)]);
}
