/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romefrontend/compiler";
import {template} from "@romefrontend/js-ast-utils";
import {jsCallExpression} from "@romefrontend/ast";

export default {
	name: "optionalChaining",
	enter(path: Path) {
		const {node} = path;

		if (node.type === "JSMemberExpression" && node.property.optional) {
			// TODO assign `node.object` to a variable and use it as a reference
			if (node.property.type === "JSComputedMemberProperty") {
				return template.expression`${node.object} == null ? undefined : ${node.object}[${node.property.value}]`;
			} else {
				return template.expression`${node.object} == null ? undefined : ${node.object}.${node.property.value}`;
			}
		}

		if (node.type === "JSOptionalCallExpression") {
			// TODO assign `node.callee` to a variable and use it as a reference
			return template.expression`${node.callee} == null ? undefined : ${jsCallExpression.create({
				callee: node.callee,
				arguments: node.arguments,
			})}`;
		}

		return node;
	},
};
