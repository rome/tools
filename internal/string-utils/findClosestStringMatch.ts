/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {findClosestMatchingStrings} from "./findClosestMatchingStrings";

export function findClosestStringMatch(
	name: string,
	matches: string[],
	minRating: number = 0.8,
): undefined | string {
	const ratings = findClosestMatchingStrings(name, matches, minRating);
	if (ratings.length === 0) {
		return undefined;
	}

	if (ratings.length === 1) {
		return ratings[0];
	}

	return ratings[0];
}
