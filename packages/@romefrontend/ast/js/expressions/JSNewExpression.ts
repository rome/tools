/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	JSSpreadElement,
	JSSuper,
	NodeBaseWithComments,
	TSTypeParameterInstantiation,
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface JSNewExpression extends NodeBaseWithComments {
	type: "JSNewExpression";
	callee: AnyJSExpression | JSSuper;
	arguments: Array<AnyJSExpression | JSSpreadElement>;
	typeArguments?: undefined | TSTypeParameterInstantiation;
	optional?: boolean;
}

export const jsNewExpression = createBuilder<JSNewExpression>(
	"JSNewExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			callee: true,
			arguments: true,
			typeArguments: true,
		},
	},
);
