/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import isValidIdentifierName from "./isValidIdentifierName";
import {
	JSIdentifier,
	JSStringLiteral,
	jsIdentifier,
	jsStringLiteral,
} from "@romejs/ast";

export default function createPropertyKey(
	name: string,
): JSIdentifier | JSStringLiteral {
	if (isValidIdentifierName(name)) {
		return jsIdentifier.quick(name);
	} else {
		return jsStringLiteral.quick(name);
	}
}
