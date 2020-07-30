/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {descriptions} from "@internal/diagnostics";
import {createVisitor, signals} from "@internal/compiler";

export default createVisitor({
	name: "js/noDebugger",
	enter(path) {
		const {node} = path;

		if (node.type === "JSDebuggerStatement") {
			return path.addFixableDiagnostic(
				{
					fixed: signals.remove,
				},
				descriptions.LINT.JS_NO_DEBUGGER,
			);
		}

		return signals.retain;
	},
});
