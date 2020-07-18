/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@romefrontend/ast";
import {Path} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {DiagnosticsDuplicateHelper} from "../../../lib/DiagnosticsDuplicateHelper";

export default {
	name: "js/noDuplicateGroupNamesInRegularExpressions",
	enter(path: Path): AnyNode {
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

		return node;
	},
};
