/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {isEscaped} from "./isEscaped";
import {ob1Coerce0} from "@internal/ob1";

export function escapeSplit(input: string, splitChar: string): Array<string> {
	const parts: Array<string> = [];
	const unescapeRegex = new RegExp(`\\\\${splitChar}`, "g");

	let buff = "";

	function push() {
		buff = buff.replace(unescapeRegex, splitChar);
		parts.push(buff);
		buff = "";
	}

	for (let i = 0; i < input.length; i++) {
		let char = input[i];

		if (!isEscaped(ob1Coerce0(i), input) && char === splitChar) {
			push();
		} else {
			buff += char;
		}
	}

	push();

	return parts;
}
