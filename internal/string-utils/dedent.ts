/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {getIndentRegex} from "./getIndentRegex";

export function dedent(
	literals: string | TemplateStringsArray,
	...values: string[]
): string {
	let str: string = "";

	if (typeof literals === "string") {
		str = literals;
	} else {
		const parts: string[] = [];

		if (literals.raw) {
			// Perform the interpolation
			for (let i = 0; i < literals.raw.length; i++) {
				parts.push(literals.raw[i]);
				if (i < values.length) {
					parts.push(values[i]);
				}
			}
		}

		str = parts.join("");
	}

	return str.replace(getIndentRegex(str), "").trim();
}
