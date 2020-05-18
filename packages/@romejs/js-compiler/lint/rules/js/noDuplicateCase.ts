/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/js-compiler";
import {AnyNode} from "@romejs/js-ast";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "noDuplicateCase",
	enter(path: Path): AnyNode {
		const {node, context} = path;

		if (node.type === "SwitchStatement") {
			const uniqueSwitchCases = new Set();

			for (const param of node.cases) {
				if (param.test && param.test.type === "StringLiteral") {
					const {test} = param;

					if (uniqueSwitchCases.has(test.value)) {
						context.addNodeDiagnostic(
							test,
							descriptions.LINT.JS_NO_DUPLICATE_CASE(test.value),
						);
					}

					uniqueSwitchCases.add(test.value);
				}
			}
		}

		return node;
	},
};
