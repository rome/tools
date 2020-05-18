/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@romejs/js-ast";
import {Path} from "@romejs/js-compiler";
import {descriptions} from "@romejs/diagnostics";
import {DiagnosticsDuplicateHelper} from "../../../lib/DiagnosticsDuplicateHelper";

export default {
	name: "noDuplicateGroupNamesInRegularExpressions",
	enter(path: Path): AnyNode {
		const {context, node} = path;

		if (node.type === "RegExpSubExpression") {
			const duplicates = new DiagnosticsDuplicateHelper(
				context,
				descriptions.LINT.JS_DUPLICATE_REGEX_GROUP_NAME,
			);

			for (const bodyItem of node.body) {
				if (bodyItem.type === "RegExpGroupCapture") {
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
