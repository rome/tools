/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/compiler";
import {AnyNode} from "@romejs/ast";
import {descriptions} from "@romejs/diagnostics";
import {DiagnosticsDuplicateHelper} from "@romejs/compiler/lib/DiagnosticsDuplicateHelper";
import {resolveIndirection, tryStaticEvaluation} from "@romejs/js-ast-utils";
import prettyFormat from "@romejs/pretty-format";

export default {
	name: "noDuplicateCase",
	enter(path: Path): AnyNode {
		const {node, context} = path;

		if (node.type === "JSSwitchStatement") {
			const duplicates = new DiagnosticsDuplicateHelper(
				context,
				descriptions.LINT.JS_NO_DUPLICATE_CASE,
			);

			for (const param of node.cases) {
				if (param.test === undefined) {
					continue;
				}

				const test = resolveIndirection(param.test, path.scope);
				const res = tryStaticEvaluation(test.node, test.scope);

				if (res.bailed && test.node.type === "JSReferenceIdentifier") {
					// We weren't able to resolve this variable further
					duplicates.addLocation(test.node.name, param.test.loc);
					continue;
				}

				// No idea what this could be
				if (res.bailed) {
					continue;
				}

				duplicates.addLocation(prettyFormat(res.value), param.test.loc);
			}

			duplicates.process();
		}

		return node;
	},
};
