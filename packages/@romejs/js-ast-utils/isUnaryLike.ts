/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';

export default function isUnaryLike(node: undefined | AnyNode): boolean {
	if (node === undefined) {
		return false;
	}

	switch (node.type) {
		case 'UnaryExpression':
		case 'SpreadElement':
		case 'SpreadProperty':
			return true;

		default:
			return false;
	}
}
