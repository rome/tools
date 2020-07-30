/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticDescription, descriptions} from "@internal/diagnostics";

const VALID_REGEX_FLAGS: Array<string> = "gmsiyu".split("");

// This is used by both rome-json and rome-js-parser to validate regex flags
export function validateRegexFlags(
	flags: string,
	onUnexpected: (
		metadata: Omit<DiagnosticDescription, "category">,
		index: number,
	) => void,
): Set<string> {
	const foundFlags: Set<string> = new Set();
	for (let i = 0; i < flags.length; i++) {
		const flag = flags[i];

		if (VALID_REGEX_FLAGS.includes(flag)) {
			if (foundFlags.has(flag)) {
				onUnexpected(descriptions.REGEX_PARSER.DUPLICATE_FLAG, i);
			} else {
				foundFlags.add(flag);
			}
		} else {
			onUnexpected(descriptions.REGEX_PARSER.INVALID_FLAG, i);
		}
	}

	return foundFlags;
}
