/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export function createIndexTracker(): IndexTracker {
	return {index: 0};
}

export type IndexTracker = {
	index: number;
};
