/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	NodeBaseWithComments,
	JSSpreadElement,
	JSSuper,
	TSTypeParameterInstantiation,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSOptionalCallExpression = NodeBaseWithComments & {
	type: "JSOptionalCallExpression";
	callee: AnyJSExpression | JSSuper;
	arguments: Array<AnyJSExpression | JSSpreadElement>;
	typeArguments?: TSTypeParameterInstantiation;
};

export const jsOptionalCallExpression = createBuilder<JSOptionalCallExpression>(
	"JSOptionalCallExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			callee: true,
			arguments: true,
			typeArguments: true,
		},
	},
);
