/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {template} from "@internal/js-ast-utils";

export default createVisitor({
	name: "nullishCoalescing",
	enter(path) {
		const {node} = path;

		if (node.type === "JSLogicalExpression" && node.operator === "??") {
			// TODO assign `node.left` to a variable and use it as a reference
			return signals.replace(
				template.expression`${node.left} == null ? ${node.right} : ${node.left}`,
			);
		}

		return signals.retain;
	},
});
