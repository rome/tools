/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {DiagnosticsDuplicateHelper} from "../../../lib/DiagnosticsDuplicateHelper";

export default createVisitor({
	name: "js/noDuplicateGroupNamesInRegularExpressions",
	enter(path) {
		const {context, node} = path;

		if (node.type === "JSRegExpSubExpression") {
			const duplicates = new DiagnosticsDuplicateHelper(
				context,
				descriptions.LINT.JS_DUPLICATE_REGEX_GROUP_NAME,
			);

			for (const bodyItem of node.body) {
				if (bodyItem.type === "JSRegExpGroupCapture") {
					const groupName = bodyItem.name;

					if (groupName !== undefined) {
						duplicates.addLocation(groupName, bodyItem.loc);
					}
				}
			}

			duplicates.process();
		}

		return signals.retain;
	},
});
