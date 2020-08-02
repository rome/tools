/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticAdvice, DiagnosticLocation} from "./types";
import {orderBySimilarity, splitLines} from "@internal/string-utils";
import stringDiff from "@internal/string-diff";
import {Position} from "@internal/parser-core";
import {ob1Get1} from "@internal/ob1";
import {StaticMarkup, markup} from "@internal/markup";

type BuildSuggestionAdviceOptions = {
	minRating?: number;
	ignoreCase?: boolean;
	formatItem?: (item: string) => StaticMarkup;
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
			language: "unknown",
			diff: stringDiff(value, topRatingRaw),
		});

		if (strings.length > 0) {
			advice.push({
				type: "log",
				category: "info",
				text: markup`Or one of these?`,
			});

			advice.push({
				type: "list",
				list: strings.map((str) => markup`${str}`),
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
			text: markup`This operation is case sensitive`,
		});
	}

	return advice;
}

// Sometimes we'll want to inline sourceText into diagnostics but the file size could be huge.
// This method trims the sourceText and pads it to fit within a certain range.
export function truncateSourceText(
	code: string,
	start: Position,
	end: Position,
): string {
	const lines = splitLines(code);

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
				text: markup`Unable to find location`,
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
			text: markup`Defined already here`,
		},
		...locationAdvice,
	];
}

export function diagnosticLocationToMarkupFilelink(
	loc: DiagnosticLocation,
): StaticMarkup {
	const {start, filename} = loc;

	if (filename === undefined) {
		return markup`unknown`;
	}

	if (start === undefined) {
		return markup`<filelink target="${filename}" />`;
	}

	return markup`<filelink target="${filename}" line="${String(start.line)}" column="${String(
		start.column,
	)}" />`;
}
