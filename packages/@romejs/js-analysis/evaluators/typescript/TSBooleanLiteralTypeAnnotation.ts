/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	TSBooleanLiteralTypeAnnotation,
	tsBooleanLiteralTypeAnnotation,
} from "@romejs/js-ast";

export default function TSBooleanLiteralTypeAnnotation(
	node: AnyNode,
	scope: Scope,
) {
	node = tsBooleanLiteralTypeAnnotation.assert(node);
	scope;
	throw new Error("unimplemented");
}
