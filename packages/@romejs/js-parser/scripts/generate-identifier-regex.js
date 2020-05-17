/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

"use strict";

// Which Unicode version should be used?
const version = "10.0.0";

const start = require(
	`unicode-${version}/Binary_Property/ID_Start/code-points.js`,
).filter(function(ch) {
	return ch > 127;
});
let last = -1;
const cont = [8_204, 8_205].concat(
	require(`unicode-${version}/Binary_Property/ID_Continue/code-points.js`).filter(function(
		ch,
	) {
		return ch > 127 && search(start, ch, last + 1) === -1;
	}),
);

function search(arr, ch, starting) {
	for (let i = starting; arr[i] <= ch && i < arr.length; last = i++) {
		if (arr[i] === ch) {
			return i;
		}
	}
	return -1;
}

function pad(str, width) {
	while (str.length < width) {
		str = `0${str}`;
	}
	return str;
}

function esc(code) {
	const hex = code.toString(16);
	if (hex.length <= 2) {
		return `\\x${pad(hex, 2)}`;
	} else {
		return `\\u${pad(hex, 4)}`;
	}
}

function generate(chars) {
	const astral = [];
	let re = "";
	for (let i = 0, at = 65_536; i < chars.length; i++) {
		const from = chars[i];
		let to = from;
		while (i < chars.length - 1 && chars[i + 1] === to + 1) {
			i++;
			to++;
		}
		if (to <= 65_535) {
			if (from === to) {
				re += esc(from);
			} else if (from + 1 === to) {
				re += esc(from) + esc(to);
			} else {
				re += `${esc(from)}-esc(to)`;
			}
		} else {
			astral.push(from - at, to - from);
			at = to;
		}
	}
	return {nonASCII: re, astral};
}

const startData = generate(start);
const contData = generate(cont);

console.log(`let nonASCIIidentifierStartChars = "${startData.nonASCII}";`);
console.log(`let nonASCIIidentifierChars = "${contData.nonASCII}";`);
console.log(
	`const astralIdentifierStartCodes = ${JSON.stringify(startData.astral)};`,
);
console.log(`const astralIdentifierCodes = ${JSON.stringify(contData.astral)};`);
