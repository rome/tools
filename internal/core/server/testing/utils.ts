/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {naturalCompare} from "@internal/string-utils";
import {CoverageDirectory} from "./types";
import {StaticMarkup, markup} from "@internal/markup";

export function sortMapKeys<T>(map: Map<string, T>): Map<string, T> {
	const sortedKeys = Array.from(map.keys()).sort(naturalCompare);
	const newMap: Map<string, T> = new Map();
	for (const key of sortedKeys) {
		const val = map.get(key);
		if (val === undefined) {
			throw new Error("Expected value");
		}
		newMap.set(key, val);
	}
	return newMap;
}

export function formatPercent(num: number): StaticMarkup {
	const str = markup`${Math.floor(num)}`;
	if (num > 80) {
		return markup`<success>${str}</success>`;
	} else if (num > 40) {
		return markup`<warn>${str}</warn>`;
	} else {
		return markup`<error>${str}</error>`;
	}
}

export function percentInsideCoverageDirectory(
	directory: CoverageDirectory,
): {
	functions: number;
	branches: number;
	lines: number;
} {
	let totalFiles = 0;
	let functions = 0;
	let branches = 0;
	let lines = 0;

	const directories = [directory];
	while (directories.length > 0) {
		const directory = directories.shift()!;

		for (const file of directory.files.values()) {
			totalFiles++;
			functions += file.functions.percent;
			branches += file.branches.percent;
			lines += file.lines.percent;
		}

		for (const subDirectory of directory.directories.values()) {
			directories.push(subDirectory);
		}
	}

	return {
		functions: totalFiles === 0 ? 100 : functions / totalFiles,
		branches: totalFiles === 0 ? 100 : branches / totalFiles,
		lines: totalFiles === 0 ? 100 : lines / totalFiles,
	};
}
