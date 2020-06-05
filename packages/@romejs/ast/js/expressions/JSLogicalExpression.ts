/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSLogicalExpression = JSNodeBase & {
	type: "JSLogicalExpression";
	operator: LogicalOperator;
	left: AnyJSExpression;
	right: AnyJSExpression;
};

export type LogicalOperator = "||" | "&&" | "??";

export const jsLogicalExpression = createBuilder<JSLogicalExpression>(
	"JSLogicalExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			left: true,
			right: true,
		},
	},
);
