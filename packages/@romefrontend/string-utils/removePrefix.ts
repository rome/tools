/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export function removePrefix(value: string, prefix: string): string {
	if (value.startsWith(prefix)) {
		return value.slice(prefix.length);
	} else {
		return value;
	}
}
