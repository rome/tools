/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticAdvice,
	DiagnosticCategoryDescription,
	DiagnosticLocation,
} from "./types";
import {orderBySimilarity, splitLines} from "@internal/string-utils";
import {Position} from "@internal/parser-core";
import {StaticMarkup, formatFileLinkInnerText, markup} from "@internal/markup";
import {joinCategoryName} from "./categories";
import {stringDiffCompressed} from "@internal/string-diff";
import {isNodeInternalPath} from "@internal/path";

type BuildSuggestionAdviceOptions = {
	minRating?: number;
	ignoreCase?: boolean;
	formatItem?: (item: string) => StaticMarkup;
};

export function buildSuggestionAdvice(
	value: string,
	items: string[],
	{minRating = 0.5, ignoreCase, formatItem}: BuildSuggestionAdviceOptions = {},
): DiagnosticAdvice[] {
	const advice: DiagnosticAdvice[] = [];

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
			diff: stringDiffCompressed(value, topRatingRaw),
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
	const fromLine = Math.max(start.line.valueOf() - 10, 0);
	const toLine = Math.max(end.line.valueOf() + 10, lines.length);

	const capturedLines = lines.slice(fromLine, toLine);
	return "\n".repeat(fromLine) + capturedLines.join("\n");
}

export function buildDuplicateLocationAdvice(
	locations: Array<undefined | DiagnosticLocation>,
): DiagnosticAdvice[] {
	const locationAdvice: DiagnosticAdvice[] = locations.map((location) => {
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

function nodeInternalDiagnosticLocationToMarkupFilelink(
	loc: DiagnosticLocation,
	innerText?: string,
): StaticMarkup {
	const {start, path} = loc;

	// Properly escape segments and remove node: prefix
	let filename = path.getSegments().map((seg) => encodeURIComponent(seg)).join(
		"/",
	).slice(5);
	if (!filename.endsWith(".js")) {
		filename += ".js";
	}

	let href = `https://github.com/nodejs/node/tree/${process.version}/lib/${filename}`;

	if (innerText === undefined) {
		innerText = formatFileLinkInnerText(path, {}, start);

		if (start !== undefined) {
			href += `#L${String(start.line.valueOf())}`;
		}
	}

	return markup`<hyperlink target="${href}">${innerText}</hyperlink>`;
}

export function diagnosticLocationToMarkupFilelink(
	loc: DiagnosticLocation,
	innerText?: string,
): StaticMarkup {
	const {start, path} = loc;

	// Link directly to GitHub for internal Node files
	if (isNodeInternalPath(path)) {
		return nodeInternalDiagnosticLocationToMarkupFilelink(loc, innerText);
	}

	if (start === undefined) {
		return markup`<filelink target="${path.join()}">${innerText ?? ""}</filelink>`;
	}

	return markup`<filelink target="${path.join()}" line="${String(
		start.line.valueOf(),
	)}" column="${String(start.column.valueOf())}">${innerText ?? ""}</filelink>`;
}

// Category value can allow arbitrary values so we need to escape bad characters
export function escapeCategoryValue(categoryValue: string): string {
	const json = JSON.stringify(categoryValue);
	if (!categoryValue.includes(")") && json === `"${categoryValue}"`) {
		// No escaping necessary
		return categoryValue;
	} else {
		return json;
	}
}

// Join category
export function formatCategoryDescription(
	{category, categoryValue}: DiagnosticCategoryDescription,
): string {
	let human: string = joinCategoryName(category);
	if (categoryValue !== undefined && categoryValue !== "") {
		human = `${human}(${escapeCategoryValue(categoryValue)})`;
	}
	return human;
}

export function appendAdviceToDiagnostic(
	diag: Diagnostic,
	advice: DiagnosticAdvice[],
): Diagnostic {
	if (advice.length === 0) {
		return diag;
	}

	return {
		...diag,
		description: {
			...diag.description,
			advice: [...(diag.description.advice || []), ...advice],
		},
	};
}

export function prependAdviceToDiagnostic(
	diag: Diagnostic,
	advice: DiagnosticAdvice[],
): Diagnostic {
	if (advice.length === 0) {
		return diag;
	}

	return {
		...diag,
		description: {
			...diag.description,
			advice: [...advice, ...(diag.description.advice || [])],
		},
	};
}
