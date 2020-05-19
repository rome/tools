/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {naturalCompare} from "./naturalCompare";

export function orderByNatural(
	strs: Array<string>,
	insensitive: boolean = true,
): Array<string> {
	return strs.sort((a, b) => naturalCompare(a, b, insensitive));
}
