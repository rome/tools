/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, NodeBaseWithComments} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSExpressionStatement extends NodeBaseWithComments {
	readonly type: "JSExpressionStatement";
	readonly expression: AnyJSExpression;
}

export const jsExpressionStatement = createQuickBuilder<
	JSExpressionStatement,
	"expression"
>(
	"JSExpressionStatement",
	"expression",
	{
		bindingKeys: {},
		visitorKeys: {
			expression: true,
		},
	},
);
