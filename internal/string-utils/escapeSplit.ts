/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ZeroIndexed} from "@internal/math";
import {isEscaped} from "./isEscaped";

export function escapeSplit(input: string, splitChar: string): string[] {
	const parts: string[] = [];
	const unescapeRegex = new RegExp(`\\\\${splitChar}`, "g");

	let buff = "";

	function push() {
		buff = buff.replace(unescapeRegex, splitChar);
		parts.push(buff);
		buff = "";
	}

	for (let i = 0; i < input.length; i++) {
		let char = input[i];

		if (!isEscaped(new ZeroIndexed(i), input) && char === splitChar) {
			push();
		} else {
			buff += char;
		}
	}

	push();

	return parts;
}
