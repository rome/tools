/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/**
 * Copyright 2014 Mozilla Foundation and contributors
 * Licensed under the New BSD license. See LICENSE or:
 * http://opensource.org/licenses/BSD-3-Clause
 */

import {Mapping} from "./types";
import {compareByGeneratedPositionsInflated} from "./util";
import {ob1Number0, ob1Number1Neg1} from "@internal/ob1";

/**
 * Determine whether mappingB is after mappingA with respect to generated
 * position.
 */
function generatedPositionAfter(mappingA: Mapping, mappingB: Mapping): boolean {
	// Optimized for most common case
	const lineA = mappingA.generated.line;
	const lineB = mappingB.generated.line;
	const columnA = mappingA.generated.column;
	const columnB = mappingB.generated.column;
	return (
		lineB > lineA ||
		(lineB === lineA && columnB >= columnA) ||
		compareByGeneratedPositionsInflated(mappingA, mappingB) <= 0
	);
}

/**
 * A data structure to provide a sorted view of accumulated mappings in a
 * performance conscious manner. It trades a negligible overhead in general
 * case for a large speedup in case of mappings being added in order.
 */
export default class MappingList {
	constructor() {
		this.array = [];
		this.sorted = true;
		this.last = {
			generated: {index: ob1Number0, line: ob1Number1Neg1, column: ob1Number0},
			// TODO: original: undefined
			original: {line: ob1Number1Neg1, column: ob1Number0},
			source: undefined,
			name: undefined,
		};
	}

	private array: Array<Mapping>;
	private sorted: boolean;

	// Serves as infimum
	private last: Mapping;

	/**
   * Add the given source mapping.
   */
	public add(mapping: Mapping) {
		if (generatedPositionAfter(this.last, mapping)) {
			this.last = mapping;
			this.array.push(mapping);
		} else {
			this.sorted = false;
			this.array.push(mapping);
		}
	}

	/**
   * Returns the flat, sorted array of mappings. The mappings are sorted by
   * generated position.
   *
   * WARNING: This method returns internal data without copying, for
   * performance. The return value must NOT be mutated, and should be treated as
   * an immutable borrow. If you want to take ownership, you must make your own
   * copy.
   */
	public toArray(): Array<Mapping> {
		if (!this.sorted) {
			this.array.sort(compareByGeneratedPositionsInflated);
			this.sorted = true;
		}
		return this.array;
	}
}
