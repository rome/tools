/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romefrontend/compiler";
import {AnyNode, jsBindingIdentifier} from "@romefrontend/ast";

export default {
	name: "paramlessCatch",
	enter(path: Path): AnyNode {
		const {node} = path;

		if (node.type === "JSCatchClause" && node.param === undefined) {
			return {
				...node,
				param: jsBindingIdentifier.create({
					name: path.scope.generateUid(),
				}),
			};
		}

		return node;
	},
};
