/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export function humanizeTime(
	ms: number,
	allowMilliseconds: boolean = false,
): string {
	if (ms === 0) {
		if (allowMilliseconds) {
			return "0ms";
		} else {
			return "0s";
		}
	}

	const s = Math.floor(ms / 1_000);
	const m = Math.floor(s / 60);
	const h = Math.floor(m / 60);

	if (h === 0 && m === 0 && s === 0) {
		if (allowMilliseconds) {
			return `${ms}ms`;
		} else {
			return `${(ms / 1_000).toFixed(2)}s`;
		}
	}

	let buf = "";
	if (h > 0) {
		buf += `${String(h)}h`;
	}
	if (m > 0) {
		buf += `${String(m % 60)}m`;
	}
	buf += `${String(s % 60)}s`;
	return buf;
}
