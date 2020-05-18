/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/js-compiler";
import {AnyNode} from "@romejs/js-ast";
import {getBindingIdentifiers} from "@romejs/js-ast-utils";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "noDupeArgs",
	enter(path: Path): AnyNode {
		const {node, context} = path;

		if (node.type === "FunctionHead") {
			const uniqueIdentifiers = new Set();

			for (const param of node.params) {
				for (const {name} of getBindingIdentifiers(param)) {
					if (uniqueIdentifiers.has(name)) {
						context.addNodeDiagnostic(
							param,
							descriptions.LINT.JAVASCRIPT_NO_DUPE_ARGS(name),
						);
					}

					uniqueIdentifiers.add(name);
				}
			}
		}

		return node;
	},
};
