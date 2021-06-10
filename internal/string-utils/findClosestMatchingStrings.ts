import {orderBySimilarity} from "./orderBySimilarity";

export function findClosestMatchingStrings(
	name: string,
	matches: string[],
	minRating: number = 0.8,
): string[] {
	if (matches.length === 0) {
		return [];
	}

	if (matches.length === 1) {
		return matches;
	}

	return orderBySimilarity(name, matches).filter((match) =>
		match.rating >= minRating
	).map((match) => match.target);
}
