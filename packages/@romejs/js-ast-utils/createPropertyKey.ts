/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import isValidIdentifierName from './isValidIdentifierName';
import {
	Identifier,
	StringLiteral,
	identifier,
	stringLiteral,
} from '@romejs/js-ast';

export default function createPropertyKey(
	name: string,
): Identifier | StringLiteral {
	if (isValidIdentifierName(name)) {
		return identifier.quick(name);
	} else {
		return stringLiteral.quick(name);
	}
}
