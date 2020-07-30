/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {RefineScope, Scope} from "../../scopes";
import {AnyNode, JSIfStatement, jsIfStatement} from "@internal/ast";
import BooleanT from "../../types/BooleanT";
import ExhaustiveT from "../../types/ExhaustiveT";
import UnionT from "../../types/UnionT";

export default function JSIfStatement(node: AnyNode, scope: Scope) {
	node =
		node.type === "JSConditionalExpression" ? node : jsIfStatement.assert(node);

	const test = scope.evaluate(node.test);
	new ExhaustiveT(scope, node, test, new BooleanT(scope, undefined));

	const hasRefinedTest: boolean = test.scope instanceof RefineScope;

	const consequentScope: Scope = hasRefinedTest ? test.scope : scope;
	const consequent = consequentScope.evaluate(node.consequent);

	if (node.alternate === undefined) {
		return consequent;
	} else {
		const alternateScope = scope.fork();

		/*if (hasRefinedTest) {
      // get bindings from 'test.scope and flip them
      for (const name of test.scope.getOwnBindingNames()) {
        const outerBinding = scope.getBinding(name);
        invariant(outerBinding !== undefined, 'expected outerBinding for %s', name);

        const refinedBinding = test.scope.getBinding(name);
        invariant(refinedBinding !== undefined, 'expected refinedBinding for %s', name);

        const opposite = new RefinedT(alternateScope, refinedBinding.originNode, outerBinding, refinedBinding);
        alternateScope.addBinding(name, opposite);
      }
    }*/
		return new UnionT(
			scope,
			undefined,
			[consequent, alternateScope.evaluate(node.alternate)],
		);
	}
}
