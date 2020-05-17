/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, REDUCE_REMOVE, TransformExitResult} from "@romejs/js-compiler";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "noEmptyCharacterClass",
	enter(path: Path): TransformExitResult {
		const {context, node} = path;

		if (node.type === "RegExpCharSet" && node.body.length === 0 && !node.invert) {
			context.addNodeDiagnostic(node, descriptions.LINT.NO_EMPTY_CHAR_SET);
			return REDUCE_REMOVE;
		}

		return node;
	},
};
