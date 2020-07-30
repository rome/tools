/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	TSNumberKeywordTypeAnnotation,
	tsNumberKeywordTypeAnnotation,
} from "@internal/ast";
import NumericT from "../../types/NumericT";

export default function TSNumberKeywordTypeAnnotation(
	node: AnyNode,
	scope: Scope,
) {
	node = tsNumberKeywordTypeAnnotation.assert(node);
	return new NumericT(scope, node);
}
