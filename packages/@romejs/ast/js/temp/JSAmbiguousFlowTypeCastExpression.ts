/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	AnyTSPrimary,
	NodeBaseWithComments,
	JSSpreadElement,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSAmbiguousFlowTypeCastExpression = NodeBaseWithComments & {
	type: "JSAmbiguousFlowTypeCastExpression";
	expression: AnyJSExpression | JSSpreadElement;
	typeAnnotation?: AnyTSPrimary;

	// This is for js-parser so that we can convert type casts to parameters

	// We should figure out some way to remove this
	optional?: boolean;
};

export const jsAmbiguousFlowTypeCastExpression = createBuilder<JSAmbiguousFlowTypeCastExpression>(
	"JSAmbiguousFlowTypeCastExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			expression: true,
			typeAnnotation: true,
		},
	},
);
