/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	JSArrayHole,
	JSSpreadElement,
	NodeBaseWithComments,
} from "@romefrontend/ast";
import {createQuickBuilder} from "../../utils";

export interface JSArrayExpression extends NodeBaseWithComments {
	type: "JSArrayExpression";
	elements: Array<JSArrayHole | AnyJSExpression | JSSpreadElement>;
}

export const jsArrayExpression = createQuickBuilder<
	JSArrayExpression,
	"elements"
>(
	"JSArrayExpression",
	"elements",
	{
		bindingKeys: {},
		visitorKeys: {
			elements: true,
		},
	},
);
