/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export type CamelCaseOptions = {
	allowShouty?: boolean;
	allowPascal?: boolean;
	forcePascal?: boolean;
};

export function removeMatch(
	inner: string,
	regex: RegExp,
	suffix: boolean,
): [string, string] {
	const match = inner.match(regex);
	if (match == null) {
		return [inner, ""];
	} else {
		const edge = match[1];
		if (suffix) {
			return [inner.slice(0, -edge.length), edge];
		} else {
			return [inner.slice(edge.length), edge];
		}
	}
}

export function toCamelCase(inner: string, opts: CamelCaseOptions = {}): string {
	// Rest of the code expects at least 1 character
	if (inner.length === 0) {
		return inner;
	}

	let prefix = "";
	let suffix = "";

	// ALLOW_STRINGS_LIKE_THIS
	if (opts.allowShouty) {
		[inner, prefix] = removeMatch(inner, /^([A-Z0-9_]+)/, false);
		[inner, suffix] = removeMatch(inner, /([A-Z0-9_]+)$/, true);
	} else if (opts.allowPascal || opts.forcePascal) {
		// Retain leading capitals only
		[inner, prefix] = removeMatch(inner, /^([A-Z]+)/, false);
	}

	// Prepend uppercase letters with a space
	inner = inner.replace(/([A-Z]+)/g, " $1");

	// Split into parts
	const parts = inner.split(/[_.\- ]+|(\d+)/g);

	// Build it
	let camel = prefix;
	let first = true;
	for (let i = 0; i < parts.length; i++) {
		let part = parts[i];
		if (part === undefined) {
			// Empty capture group
			continue;
		}

		// Don't capitalize the first part unless we want pascal case
		if (!first || (opts.forcePascal && prefix === "")) {
			// Needs at least one
			if (part.length > 0) {
				part = part[0].toUpperCase() + part.slice(1);
			}
		}

		if (first) {
			first = false;
		}

		camel += part;
	}
	camel += suffix;
	return camel;
}
