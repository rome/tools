/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {DiagnosticsDuplicateHelper} from "@internal/compiler/lib/DiagnosticsDuplicateHelper";
import {resolveIndirection, tryStaticEvaluation} from "@internal/js-ast-utils";
import {prettyFormatToString} from "@internal/pretty-format";

export default createVisitor({
	name: "js/noDuplicateCase",
	enter(path) {
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

				duplicates.addLocation(prettyFormatToString(res.value), param.test.loc);
			}

			duplicates.process();
		}

		return signals.retain;
	},
});
