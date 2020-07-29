/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface JSLogicalExpression extends NodeBaseWithComments {
	readonly type: "JSLogicalExpression";
	readonly operator: LogicalOperator;
	readonly left: AnyJSExpression;
	readonly right: AnyJSExpression;
}

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
