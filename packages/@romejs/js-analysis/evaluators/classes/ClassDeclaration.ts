/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, ClassDeclaration, classDeclaration} from "@romejs/js-ast";
import ClassExpression from "./ClassExpression";

export default function ClassDeclaration(node: AnyNode, scope: Scope) {
	node = classDeclaration.assert(node);
	const type = ClassExpression(node, scope);
	if (node.id) {
		scope.addBinding(node.id.name, type);
	}
	return type;
}
