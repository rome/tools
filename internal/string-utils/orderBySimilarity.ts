/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

function getMap<Key, Value>(
	map: Map<Key, NonNullable<Value>>,
	key: Key,
	defaultValue?: NonNullable<Value>,
): NonNullable<Value> {
	const existing = map.get(key);

	if (existing === undefined) {
		if (defaultValue === undefined) {
			throw new Error("Key didn't exist and no defaultValue passed");
		}

		map.set(key, defaultValue);
		return defaultValue;
	} else {
		return existing;
	}
}

/**
 * Forked from the project https://github.com/aceakash/string-similarity by Akash K, licensed under ISC
 */
export function compareTwoStrings(aStr: string, bStr: string): number {
	const a = aStr.replace(/\s+/g, "");
	const b = bStr.replace(/\s+/g, "");

	// If both are empty strings
	if (!a.length && !b.length) {
		return 1;
	}

	// If only one is empty string
	if (!a.length || !b.length) {
		return 0;
	}

	// Identical
	if (a === b) {
		return 1;
	}

	// Both are 1-letter strings
	if (a.length === 1 && b.length === 1) {
		return 0;
	}

	// If either is a 1-letter string
	if (a.length < 2 || b.length < 2) {
		return 0;
	}

	let firstBigrams: Map<string, number> = new Map();
	for (let i = 0; i < a.length - 1; i++) {
		const bigram = a.substring(i, i + 2);

		const count = firstBigrams.has(bigram)
			? getMap(firstBigrams, bigram) + 1
			: 1;
		if (count === undefined) {
			throw new Error("Already used has() above");
		}

		firstBigrams.set(bigram, count);
	}

	let intersectionSize: number = 0;
	for (let i = 0; i < b.length - 1; i++) {
		const bigram = b.substring(i, i + 2);

		const count = getMap(firstBigrams, bigram, 0);
		if (count === undefined) {
			throw new Error("Already used has() above");
		}

		if (count > 0) {
			firstBigrams.set(bigram, count - 1);
			intersectionSize++;
		}
	}

	return 2 * intersectionSize / (a.length + b.length - 2);
}

export type Rating = {
	target: string;
	rating: number;
};

export type Ratings = Array<Rating>;

type OrderBySimilarityOptions = {
	minRating?: number;
	ignoreCase?: boolean;
};

export function orderBySimilarity(
	compareStr: string,
	targets: Array<string>,
	{minRating, ignoreCase = false}: OrderBySimilarityOptions = {},
): Ratings {
	if (targets.length === 0) {
		return [];
	}

	// Calculate the rating for each target string
	const ratings: Ratings = Array.from(
		targets,
		(target: string): Rating => {
			if (ignoreCase) {
				return {
					target,
					rating: compareTwoStrings(
						compareStr.toLowerCase(),
						target.toLowerCase(),
					),
				};
			}

			return {
				target,
				rating: compareTwoStrings(compareStr, target),
			};
		},
	);

	// Sort ratings, with the highest at the beginning
	const sortedRatings: Ratings = ratings.sort((a, b) => {
		return b.rating - a.rating;
	}).filter((item) => minRating === undefined || item.rating >= minRating);

	return sortedRatings;
}
