/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Number0, ob1Dec, ob1Get0} from "@internal/ob1";
export function isEscaped(index: Number0, input: string): boolean {
	const prevChar = input[ob1Get0(index) - 1];

	if (prevChar === "\\") {
		return !isEscaped(ob1Dec(index), input);
	} else {
		return false;
	}
}
