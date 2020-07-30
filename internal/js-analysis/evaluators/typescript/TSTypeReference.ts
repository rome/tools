/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, TSTypeReference, tsTypeReference} from "@internal/ast";

export default function TSTypeReference(node: AnyNode, scope: Scope) {
	node = tsTypeReference.assert(node);
	scope;
	throw new Error("unimplemented");
}
