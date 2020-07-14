/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export function dedent(
	literals: string | TemplateStringsArray,
	...values: Array<string>
): string {
	let string: string = "";

	if (typeof literals === "string") {
		string = literals;
	} else {
		const parts: Array<string> = [];

		if (literals.raw) {
			// Perform the interpolation
			for (let i = 0; i < literals.raw.length; i++) {
				parts.push(literals.raw[i]);
				if (i < values.length) {
					parts.push(values[i]);
				}
			}
		}

		string = parts.join("");
	}

	// Find min indentation
	const match = string.match(/^[ \t]*(?=\S)/gm);
	if (match === null) {
		return string;
	}

	const indent = Math.min(...match.map((x) => x.length));

	// Remove indentation
	return string.replace(new RegExp(`^[ \\t]{${indent}}`, "gm"), "").trim();
}
