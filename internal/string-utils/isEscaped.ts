/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ZeroIndexed} from "@internal/math";

export function isEscaped(index: ZeroIndexed, input: string): boolean {
	const prevChar = input[index.valueOf() - 1];

	if (prevChar === "\\") {
		return !isEscaped(index.decrement(), input);
	} else {
		return false;
	}
}
