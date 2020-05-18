/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from "@romejs/js-compiler";
import {referenceIdentifier} from "@romejs/js-ast";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "sparseArray",
	enter(path: Path): TransformExitResult {
		const {node, parent} = path;

		if (node.type === "ArrayHole" && parent.type === "ArrayExpression") {
			return path.context.addFixableDiagnostic(
				{
					old: node,
					fixed: referenceIdentifier.create({name: "undefined"}),
				},
				descriptions.LINT.JAVASCRIPT_SPARSE_ARRAY,
			);
		}

		return node;
	},
};
