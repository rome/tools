/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/js-compiler";
import {AnyNode, jsReferenceIdentifier} from "@romejs/ast";

export default {
	name: "asyncImport",
	enter(path: Path): AnyNode {
		const {node} = path;

		if (node.type === "JSCallExpression" && node.callee.type === "JSImportCall") {
			return {
				...node,
				callee: jsReferenceIdentifier.create({
					name: "require",
				}),
			};
		}

		return node;
	},
};
