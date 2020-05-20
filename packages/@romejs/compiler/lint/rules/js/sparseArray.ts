/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from "@romejs/compiler";
import {jsReferenceIdentifier} from "@romejs/ast";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "sparseArray",
	enter(path: Path): TransformExitResult {
		const {node, parent} = path;

		if (node.type === "JSArrayHole" && parent.type === "JSArrayExpression") {
			return path.context.addFixableDiagnostic(
				{
					old: node,
					fixed: jsReferenceIdentifier.create({name: "undefined"}),
				},
				descriptions.LINT.JS_SPARSE_ARRAY,
			);
		}

		return node;
	},
};
