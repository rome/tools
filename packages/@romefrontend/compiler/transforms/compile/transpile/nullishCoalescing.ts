/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romefrontend/compiler";
import {template} from "@romefrontend/js-ast-utils";

export default {
	name: "nullishCoalescing",
	enter(path: Path) {
		const {node} = path;

		if (node.type === "JSLogicalExpression" && node.operator === "??") {
			// TODO assign `node.left` to a variable and use it as a reference
			return template.expression`${node.left} == null ? ${node.right} : ${node.left}`;
		}

		return node;
	},
};
