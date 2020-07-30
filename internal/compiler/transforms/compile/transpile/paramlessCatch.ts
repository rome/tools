/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {jsBindingIdentifier} from "@internal/ast";

export default createVisitor({
	name: "paramlessCatch",
	enter(path) {
		const {node} = path;

		if (node.type === "JSCatchClause" && node.param === undefined) {
			return signals.replace({
				...node,
				param: jsBindingIdentifier.create({
					name: path.scope.generateUid(),
				}),
			});
		}

		return signals.retain;
	},
});
