/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@romejs/js-ast";
import {Path} from "@romejs/js-compiler";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "noExplicitAny",
	enter(path: Path): AnyNode {
		const {context, node} = path;

		if (node.type === "AnyKeywordTypeAnnotation") {
			context.addNodeDiagnostic(node, descriptions.LINT.NO_EXPLICIT_ANY);
		}

		return node;
	},
};
