/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import prettyFormat from '@romejs/pretty-format';

export function format(value: unknown): string {
	if (typeof value === 'string') {
		return value;
	} else {
		return prettyFormat(value);
	}
}
