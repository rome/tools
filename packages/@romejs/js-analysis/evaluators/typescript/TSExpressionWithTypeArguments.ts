/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	TSExpressionWithTypeArguments,
	tsExpressionWithTypeArguments,
} from "@romejs/ast";

export default function TSExpressionWithTypeArguments(node: AnyNode) {
	node = tsExpressionWithTypeArguments.assert(node);
	throw new Error("unimplemented");
}
