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
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSOptionalCallExpression extends NodeBaseWithComments {
	readonly type: "JSOptionalCallExpression";
	readonly callee: AnyJSExpression | JSSuper;
	readonly arguments: Array<AnyJSExpression | JSSpreadElement>;
	readonly typeArguments?: TSTypeParameterInstantiation;
}

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
