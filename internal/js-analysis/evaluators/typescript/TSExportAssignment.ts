/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, TSExportAssignment, tsExportAssignment} from "@internal/ast";

export default function TSExportAssignment(node: AnyNode, scope: Scope) {
	node = tsExportAssignment.assert(node);
	scope;
	throw new Error("unimplemented");
}
