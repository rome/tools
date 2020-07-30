/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {getNodeReferenceParts} from "./getNodeReferenceParts";
import {isIdentifierish} from "./isIdentifierish";

const splitCache: Map<string, SplitResult> = new Map();

type SplitResult = {
	hasDoubleStar: boolean;
	parts: Array<string>;
};

function split(str: string): SplitResult {
	const cached = splitCache.get(str);
	if (cached !== undefined) {
		return cached;
	}

	const parts = str.split(".");

	let hasDoubleStar = false;
	for (const part of parts) {
		if (part === "**") {
			hasDoubleStar = true;
			break;
		}
	}

	const result: SplitResult = {parts, hasDoubleStar};
	splitCache.set(str, result);
	return result;
}

export function doesNodeMatchPattern(
	node: undefined | AnyNode,
	match: string,
): boolean {
	if (node === undefined) {
		return false;
	}

	// Not a member expression
	if (
		node.type !== "JSMemberExpression" &&
		node.type !== "JSXMemberExpression" &&
		!isIdentifierish(node)
	) {
		return false;
	}

	const {parts: expectedParts, hasDoubleStar} = split(match);

	// Fast path for single part pattern matching
	if (expectedParts.length === 1 && expectedParts[0] !== "*" && !hasDoubleStar) {
		return isIdentifierish(node) && node.name === expectedParts[0];
	}

	const {bailed, parts: actualParts} = getNodeReferenceParts(node);

	// Bailed will be true if we were unable to derive a name for one of the parts
	if (bailed && !hasDoubleStar) {
		return false;
	}

	// If there's less parts than the amount we expect then it's never going to match
	if (actualParts.length < expectedParts.length) {
		return false;
	}

	// I there's more parts than we expect then it's never going to match either
	if (!hasDoubleStar && actualParts.length > expectedParts.length) {
		return false;
	}

	let nextActualIndex = 0;
	let nextExpectedIndex = 0;

	// Loop over the parts we received and match them
	while (nextActualIndex < actualParts.length) {
		// If we have no more expected parts then we can't possibly match it
		if (nextActualIndex >= expectedParts.length) {
			return false;
		}

		const actual = actualParts[nextActualIndex].value;
		nextActualIndex++;

		const expected = expectedParts[nextExpectedIndex];
		nextExpectedIndex++;

		// A star part can accept anything
		if (expected === "*") {
			continue;
		}

		if (expected === "**") {
			// Ran out of matches but we've accepted the current part
			if (nextExpectedIndex >= expectedParts.length) {
				return true;
			}

			const next = expectedParts[nextExpectedIndex];
			nextExpectedIndex++;

			if (next === "*" || next === "**") {
				throw new Error(
					`The next expected part was ${next} but this isn't allowed since we're processing a double star`,
				);
			}

			let found = false;

			// Eat as many parts until we find the next expected part
			while (nextActualIndex < actualParts.length) {
				const actual = actualParts[nextActualIndex].value;
				nextActualIndex++;
				if (actual === next) {
					found = true;
					break;
				}
			}

			if (found) {
				continue;
			} else {
				return false;
			}
		}

		if (expected !== actual) {
			return false;
		}
	}

	return true;
}
