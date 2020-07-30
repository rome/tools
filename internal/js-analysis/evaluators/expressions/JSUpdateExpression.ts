/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, JSUpdateExpression, jsUpdateExpression} from "@internal/ast";
import NumericT from "../../types/NumericT";
import ExhaustiveT from "../../types/ExhaustiveT";

export default function JSUpdateExpression(node: AnyNode, scope: Scope) {
	node = jsUpdateExpression.assert(node);
	const type = new NumericT(scope, node);
	new ExhaustiveT(scope, node.argument, scope.evaluate(node.argument), type);
	return type;
}
