/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyExpression,
	JSNodeBase,
	SpreadElement,
	Super,
	TSTypeParameterInstantiation,
} from "../index";
import {createBuilder} from "../utils";

export type OptionalCallExpression = JSNodeBase & {
	type: "OptionalCallExpression";
	callee: AnyExpression | Super;
	arguments: Array<AnyExpression | SpreadElement>;
	typeArguments?: TSTypeParameterInstantiation;
};

export const optionalCallExpression = createBuilder<OptionalCallExpression>(
	"OptionalCallExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			callee: true,
			arguments: true,
			typeArguments: true,
		},
	},
);
