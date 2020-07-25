/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/*
 * Copyright 2011 Mozilla Foundation and contributors
 * Licensed under the New BSD license. See LICENSE or:
 * http://opensource.org/licenses/BSD-3-Clause
 */

/**
 * A data structure which is a combination of an array and a set. Adding a new
 * member is O(1), testing for membership is O(1), and finding the index of an
 * element is O(1). Removing elements from the set is not supported. Only
 * strings are supported for membership.
 */
export default class ArraySet {
	constructor() {
		this.array = [];
		this.set = new Map();
	}

	array: Array<string>;
	set: Map<string, number>;

	/**
   * Static method for creating ArraySet instances from 'an existing array.
   */
	static fromArray(array: Array<string>, allowDuplicates: boolean): ArraySet {
		const set = new ArraySet();
		for (const item of array) {
			set.add(item, allowDuplicates);
		}
		return set;
	}

	/**
   * Add the given string to this set.
   */
	add(str: string, allowDuplicates?: boolean): void {
		const isDuplicate = this.has(str);
		const idx = this.array.length;

		if (isDuplicate === false || allowDuplicates === true) {
			this.array.push(str);
		}

		if (isDuplicate === false) {
			this.set.set(str, idx);
		}
	}

	/**
   * Is the given string a member of this set?
   */
	has(str: string): boolean {
		return this.set.has(str);
	}

	/**
   * What is the index of the given string in the array?
   */
	indexOf(str: string): number {
		const idx = this.set.get(str);
		if (idx === undefined || idx < 0) {
			throw new Error(`${str} is not in the set`);
		}
		return idx;
	}

	/**
   * What is the element at the given index?
   */
	at(idx: number): string {
		if (idx >= 0 && idx < this.array.length) {
			return this.array[idx];
		} else {
			throw new Error(`No element indexed by ${idx}`);
		}
	}

	/**
   * Returns the array representation of this set (which has the proper indices
   * indicated by indexOf). Note that this is a copy of the internal array used
   * for storing the members so that no one can mess with internal state.
   */
	toArray(): Array<string> {
		return this.array.slice();
	}
}
