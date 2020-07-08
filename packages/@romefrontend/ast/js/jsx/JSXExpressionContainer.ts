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
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type JSXExpressionContainer = NodeBaseWithComments & {
	type: "JSXExpressionContainer";
	expression: AnyJSExpression | JSXEmptyExpression;
};

export const jsxExpressionContainer = createBuilder<JSXExpressionContainer>(
	"JSXExpressionContainer",
	{
		bindingKeys: {},
		visitorKeys: {
			expression: true,
		},
	},
);
