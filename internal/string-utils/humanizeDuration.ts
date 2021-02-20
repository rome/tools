/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

const shortSuffixes = {
	ms: "ms",
	s: "s",
	m: "m",
	h: "h",
};

const longSuffixes = {
	ms: "millisecond",
	s: "second",
	m: "minute",
	h: "hour",
};

function format(
	key: keyof typeof shortSuffixes,
	num: string | number,
	longform: boolean,
): string {
	const str = String(num);
	if (longform) {
		let suffix = longSuffixes[key];
		if (str !== "1") {
			suffix += "s";
		}
		return `${str} ${suffix} `;
	} else {
		return `${str}${shortSuffixes[key]}`;
	}
}

export function humanizeDuration(
	ms: number,
	{allowMilliseconds = false, longform = false, secondFractionDigits = 2}: {
		allowMilliseconds?: boolean;
		secondFractionDigits?: number;
		longform?: boolean;
	} = {},
): string {
	if (ms === 0) {
		if (allowMilliseconds) {
			return format("ms", 0, longform);
		} else {
			return format("s", 0, longform);
		}
	}

	const s = Math.floor(ms / 1_000);
	const m = Math.floor(s / 60);
	const h = Math.floor(m / 60);

	if (h === 0 && m === 0 && s === 0) {
		if (allowMilliseconds) {
			return format("ms", ms, longform);
		} else {
			return format("s", (ms / 1_000).toFixed(secondFractionDigits), longform);
		}
	}

	let buf = "";

	if (h > 0) {
		buf += format("h", h, longform);
	}

	if (m > 0) {
		buf += format("m", m % 60, longform);
	}

	if (allowMilliseconds) {
		buf += format(
			"s",
			(ms / 1_000 % 60).toFixed(secondFractionDigits),
			longform,
		);
	} else {
		buf += format("s", s % 60, longform);
	}

	if (longform) {
		buf = buf.trimRight();
	}

	return buf;
}
