/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

const SCIENTIFIC_NOTATION = /e/i;

export function humanizeNumber(num: bigint | number, sep: string = "_"): string {
	let str = String(num);

	if (num < 1_000) {
		return str;
	}

	if (SCIENTIFIC_NOTATION.test(str)) {
		return str;
	}

	const decimals = str.split(".");

	let intChars: Array<string> = String(decimals.shift()).split("");
	let intParts: Array<string> = [];

	while (intChars.length > 0) {
		const part = intChars.slice(-3).join("");
		intParts.unshift(part);

		intChars = intChars.slice(0, -3);
	}

	return [intParts.join(sep), ...decimals].join(".");
}
