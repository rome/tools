/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

export default createVisitor({
	name: "regex/noEmptyCharacterClass",
	enter(path) {
		const {context, node} = path;

		if (
			node.type === "JSRegExpCharSet" &&
			node.body.length === 0 &&
			!node.invert
		) {
			context.addNodeDiagnostic(node, descriptions.LINT.REGEX_NO_EMPTY_CHAR_SET);
			return signals.remove;
		}

		return signals.retain;
	},
});
