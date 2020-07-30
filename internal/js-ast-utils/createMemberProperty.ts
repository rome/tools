/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSComputedMemberProperty,
	JSStaticMemberProperty,
	jsComputedMemberProperty,
	jsIdentifier,
	jsStaticMemberProperty,
	jsStringLiteral,
} from "@internal/ast";
import {isValidIdentifierName} from "./isValidIdentifierName";

export function createMemberProperty(
	name: string,
): JSStaticMemberProperty | JSComputedMemberProperty {
	if (isValidIdentifierName(name)) {
		return jsStaticMemberProperty.quick(jsIdentifier.quick(name));
	} else {
		return jsComputedMemberProperty.quick(jsStringLiteral.quick(name));
	}
}
