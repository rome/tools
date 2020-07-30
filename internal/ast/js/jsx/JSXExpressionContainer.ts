/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	JSXEmptyExpression,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSXExpressionContainer extends NodeBaseWithComments {
	readonly type: "JSXExpressionContainer";
	readonly expression: AnyJSExpression | JSXEmptyExpression;
}

export const jsxExpressionContainer = createBuilder<JSXExpressionContainer>(
	"JSXExpressionContainer",
	{
		bindingKeys: {},
		visitorKeys: {
			expression: true,
		},
	},
);
