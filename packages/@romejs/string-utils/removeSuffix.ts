/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export function removeSuffix(value: string, suffix: string): string {
	if (value.endsWith(suffix)) {
		return value.slice(0, -suffix.length);
	} else {
		return value;
	}
}
