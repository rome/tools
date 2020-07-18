/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {descriptions} from "@romefrontend/diagnostics";
import {Path, REDUCE_REMOVE, TransformExitResult} from "@romefrontend/compiler";

export default {
	name: "js/noDebugger",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (node.type === "JSDebuggerStatement") {
			return path.context.addFixableDiagnostic(
				{
					old: node,
					fixed: REDUCE_REMOVE,
				},
				descriptions.LINT.JS_NO_DEBUGGER,
			);
		}

		return node;
	},
};
