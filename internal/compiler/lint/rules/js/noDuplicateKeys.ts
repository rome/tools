/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createLintVisitor, signals} from "@internal/compiler";
import {JSObjectMethod, JSObjectProperty} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";
import {DiagnosticsDuplicateHelper} from "../../../lib/DiagnosticsDuplicateHelper";

function extractPropertyKey(
	node: JSObjectProperty | JSObjectMethod,
): string | undefined {
	if (node.key.type === "JSStaticPropertyKey") {
		const {value} = node.key;

		if (value.type === "JSPrivateName") {
			return value.id.name;
		}

		if (value.type === "JSIdentifier") {
			return value.name;
		}

		return String(value.value);
	}

	return undefined;
}

export default createLintVisitor({
	name: "js/noDuplicateKeys",
	enter(path) {
		const {node, context} = path;

		if (node.type === "JSObjectExpression") {
			const duplicates = new DiagnosticsDuplicateHelper(
				context,
				descriptions.LINT.JS_NO_DUPLICATE_KEYS,
			);

			for (const prop of node.properties) {
				if (prop.type === "JSSpreadProperty") {
					continue;
				}

				const key = extractPropertyKey(prop);
				if (key !== undefined) {
					duplicates.addLocation(key, prop.key.loc);
				}
			}

			duplicates.process();
		}

		return signals.retain;
	},
});
