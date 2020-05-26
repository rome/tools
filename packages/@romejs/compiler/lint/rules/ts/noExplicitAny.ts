/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@romejs/ast";
import {Path} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "tsNoExplicitAny",
	enter(path: Path): AnyNode {
		const {context, node} = path;

		if (node.type === "TSAnyKeywordTypeAnnotation") {
			context.addNodeDiagnostic(node, descriptions.LINT.TS_NO_EXPLICIT_ANY);
		}

		return node;
	},
};
