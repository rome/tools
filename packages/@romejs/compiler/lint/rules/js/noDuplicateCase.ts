/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/compiler";
import {AnyNode} from "@romejs/ast";
import {descriptions} from "@romejs/diagnostics";

function getRawValue(node: AnyNode): string | undefined {
    switch(node.type) {
        case "JSStringLiteral":
            return `'${node.value}'`;
        case "JSBigIntLiteral":
            return `${node.value}n`;
        case "JSNumericLiteral":
            return String(node.value);
        case "JSReferenceIdentifier":
            return String(node.name);
        case "JSNullLiteral":
            return "null";
    }
    return undefined;
}

export default {
	name: "noDuplicateCase",
	enter(path: Path): AnyNode {
		const {node, context} = path;

		if (node.type === "JSSwitchStatement") {
			const uniqueSwitchCases = new Set();

			for (const param of node.cases) {
                const rawValue = param.test && getRawValue(param.test);

				if (rawValue) {
					const {test} = param;

					if (uniqueSwitchCases.has(rawValue)) {
						context.addNodeDiagnostic(
							test,
							descriptions.LINT.JS_NO_DUPLICATE_CASE(rawValue),
						);
					}

					uniqueSwitchCases.add(rawValue);
				}
			}
		}

		return node;
	},
};
