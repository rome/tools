/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {template} from "@internal/js-ast-utils";
import {jsCallExpression} from "@internal/ast";

export default createVisitor({
	name: "optionalChaining",
	enter(path) {
		const {node} = path;

		if (node.type === "JSMemberExpression" && node.property.optional) {
			// TODO assign `node.object` to a variable and use it as a reference
			if (node.property.type === "JSComputedMemberProperty") {
				return signals.replace(
					template.expression`${node.object} == null ? undefined : ${node.object}[${node.property.value}]`,
				);
			} else {
				return signals.replace(
					template.expression`${node.object} == null ? undefined : ${node.object}.${node.property.value}`,
				);
			}
		}

		if (node.type === "JSOptionalCallExpression") {
			// TODO assign `node.callee` to a variable and use it as a reference
			return signals.replace(
				template.expression`${node.callee} == null ? undefined : ${jsCallExpression.create({
					callee: node.callee,
					arguments: node.arguments,
				})}`,
			);
		}

		return signals.retain;
	},
});
