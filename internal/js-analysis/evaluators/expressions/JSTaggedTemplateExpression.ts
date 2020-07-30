/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	JSTaggedTemplateExpression,
	jsTaggedTemplateExpression,
} from "@internal/ast";

export default function JSTaggedTemplateExpression(node: AnyNode, scope: Scope) {
	node = jsTaggedTemplateExpression.assert(node);
	scope;
	throw new Error("unimplemented");
}
