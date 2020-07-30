/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/**
 * Copyright 2011 Mozilla Foundation and contributors
 * Licensed under the New BSD license. See LICENSE or:
 * http://opensource.org/licenses/BSD-3-Clause
 */

import {Mapping} from "./types";
import {ob1Get0, ob1Get1} from "@internal/ob1";

function strcmp(a: undefined | string, b: undefined | string): number {
	if (a === b) {
		return 0;
	}

	if (a === undefined) {
		return 1;
	}

	if (b === undefined) {
		return -1;
	}

	if (a > b) {
		return 1;
	}

	return -1;
}

/**
 * Comparator between two mappings with inflated source and name strings where
 * the generated positions are compared.
 */
export function compareByGeneratedPositionsInflated(
	mappingA: Mapping,
	mappingB: Mapping,
): number {
	let cmp: number =
		ob1Get1(mappingA.generated.line) - ob1Get1(mappingB.generated.line);
	if (cmp !== 0) {
		return cmp;
	}

	cmp = ob1Get0(mappingA.generated.column) - ob1Get0(mappingB.generated.column);
	if (cmp !== 0) {
		return cmp;
	}

	cmp = strcmp(mappingA.source, mappingB.source);
	if (cmp !== 0) {
		return cmp;
	}

	if (mappingA.original == null) {
		if (mappingB.original != null) {
			return 1;
		}
	} else if (mappingB.original == null) {
		return -1;
	} else {
		cmp = ob1Get1(mappingA.original.line) - ob1Get1(mappingB.original.line);
		if (cmp !== 0) {
			return cmp;
		}

		cmp = ob1Get0(mappingA.original.column) - ob1Get0(mappingB.original.column);
		if (cmp !== 0) {
			return cmp;
		}
	}

	return strcmp(mappingA.name, mappingB.name);
}

/**
 * Make a path relative to a URL or another path.
 */
export function toRelativeUrl(root: string, path: string): string {
	if (root === "") {
		root = ".";
	}

	root = root.replace(/\/$/, "");

	// It is possible for the path to be above the root. In this case, simply

	// checking whether the root is a prefix of the path won't work. Instead, we

	// need to remove components from the root one by one, until either we find

	// a prefix that fits, or we run out of components to remove.
	let level = 0;
	while (path.indexOf(`${root}/`) !== 0) {
		const index = root.lastIndexOf("/");
		if (index < 0) {
			return path;
		}

		// If the only part of the root that is left is the scheme (i.e. http://,

		// file:///, etc.), one or more slashes (/), or simply nothing at all, we

		// have exhausted all components, so the path is not relative to the root.
		root = root.slice(0, index);
		if (root.match(/^([^\/]+:\/)?\/*$/)) {
			return path;
		}

		++level;
	}

	// Make sure we add a '../' for each component we removed from the root.

	return "../".repeat(level) + path.substr(root.length + 1);
}
