/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {jsReferenceIdentifier} from "@internal/ast";

export default createVisitor({
	name: "asyncImport",
	enter(path) {
		const {node} = path;

		if (node.type === "JSCallExpression" && node.callee.type === "JSImportCall") {
			return signals.replace({
				...node,
				callee: jsReferenceIdentifier.create({
					name: "require",
				}),
			});
		}

		return signals.retain;
	},
});
