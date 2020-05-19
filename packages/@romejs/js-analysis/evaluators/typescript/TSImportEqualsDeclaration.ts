/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	TSImportEqualsDeclaration,
	tsImportEqualsDeclaration,
} from "@romejs/ast";

export default function TSImportEqualsDeclaration(node: AnyNode) {
	node = tsImportEqualsDeclaration.assert(node);
	throw new Error("unimplemented");
}
