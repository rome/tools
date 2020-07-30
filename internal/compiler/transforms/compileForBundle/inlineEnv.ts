/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {doesNodeMatchPattern} from "@internal/js-ast-utils";
import {jsStringLiteral} from "@internal/ast";

export default createVisitor({
	name: "inlineEnv",
	enter(path) {
		const {node} = path;

		if (
			node.type === "JSMemberExpression" &&
			node.property.value.type === "JSIdentifier" &&
			node.property.value.name === "NODE_ENV" &&
			!path.scope.hasBinding("process") &&
			doesNodeMatchPattern(node, "process.env.NODE_ENV")
		) {
			return signals.replace(
				jsStringLiteral.create({
					value: "development",
				}),
			);
		}

		return signals.retain;
	},
});
