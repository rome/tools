/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticAdvice, DiagnosticLocation} from "./types";
import {orderBySimilarity} from "@romefrontend/string-utils";
import stringDiff from "@romefrontend/string-diff";
import {Position} from "@romefrontend/parser-core";
import {ob1Get1} from "@romefrontend/ob1";
import {NEWLINE} from "@romefrontend/js-parser-utils";
import {escapeMarkup, markup} from "@romefrontend/string-markup";

type BuildSuggestionAdviceOptions = {
	minRating?: number;
	ignoreCase?: boolean;
	formatItem?: (item: string) => string;
};

export function buildSuggestionAdvice(
	value: string,
	items: Array<string>,
	{minRating = 0.5, ignoreCase, formatItem}: BuildSuggestionAdviceOptions = {},
): DiagnosticAdvice {
	const advice: DiagnosticAdvice = [];

	const ratings = orderBySimilarity(
		value,
		items,
		{
			minRating,
			formatItem,
			ignoreCase,
		},
	);

	const strings = ratings.map((item) => {
		const {target} = item;
		if (formatItem === undefined) {
			return target;
		} else {
			return formatItem(target);
		}
	});

	const topRatingFormatted = strings.shift();
	if (topRatingFormatted === undefined) {
		return advice;
	}

	// Raw rating that hasn't been formatted
	const topRatingRaw = ratings[0].target;

	if (topRatingRaw === value) {
		// TODO produce a better example
	}

	// If there's only 2 suggestions then just say "Did you mean A or B?" rather than printing the list
	if (strings.length === 1) {
		advice.push({
			type: "log",
			category: "info",
			text: markup`Did you mean <emphasis>${topRatingFormatted}</emphasis> or <emphasis>${strings[0]}</emphasis>?`,
		});
	} else {
		advice.push({
			type: "log",
			category: "info",
			text: markup`Did you mean <emphasis>${topRatingFormatted}</emphasis>?`,
		});

		advice.push({
			type: "diff",
			diff: stringDiff(value, topRatingRaw),
		});

		if (strings.length > 0) {
			advice.push({
				type: "log",
				category: "info",
				text: "Or one of these?",
			});

			advice.push({
				type: "list",
				list: strings.map((str) => escapeMarkup(str)),
				truncate: true,
			});
		}
	}

	// TODO check if ANY of the suggestions match
	if (
		topRatingRaw !== value &&
		topRatingRaw.toLowerCase() === value.toLowerCase()
	) {
		advice.push({
			type: "log",
			category: "warn",
			text: "This operation is case sensitive",
		});
	}

	return advice;
}

// Sometimes we'll have big blobs of JS in a diagnostic when we'll only show a snippet. This method pads it out then truncates the rest for efficient transmission. We will have crappy ANSI formatting in the console and elsewhere but for places where we need to truncate we probably don't care (generated code).
export function truncateSourceText(
	code: string,
	start: Position,
	end: Position,
): string {
	const lines = code.split(NEWLINE);

	// Pad the starting and ending lines by 10
	const fromLine = Math.max(ob1Get1(start.line) - 10, 0);
	const toLine = Math.max(ob1Get1(end.line) + 10, lines.length);

	const capturedLines = lines.slice(fromLine, toLine);
	return "\n".repeat(fromLine) + capturedLines.join("\n");
}

export function buildDuplicateLocationAdvice(
	locations: Array<undefined | DiagnosticLocation>,
): DiagnosticAdvice {
	const locationAdvice: DiagnosticAdvice = locations.map((location) => {
		if (location === undefined) {
			return {
				type: "log",
				category: "warn",
				text: "Unable to find location",
			};
		} else {
			return {
				type: "frame",
				location,
			};
		}
	});

	return [
		{
			type: "log",
			category: "info",
			text: "Defined already here",
		},
		...locationAdvice,
	];
}

export function diagnosticLocationToMarkupFilelink(
	loc: DiagnosticLocation,
): string {
	const {start, filename} = loc;

	if (filename === undefined) {
		return "unknown";
	}

	if (start === undefined) {
		return markup`<filelink target="${filename}" />`;
	}

	return markup`<filelink target="${filename}" line="${start.line}" column="${start.column}" />`;
}
