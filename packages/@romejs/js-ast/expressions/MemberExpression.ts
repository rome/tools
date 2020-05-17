/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyExpression,
	ComputedMemberProperty,
	JSNodeBase,
	StaticMemberProperty,
	Super,
} from "../index";
import {createBuilder} from "../utils";

export type MemberExpression = JSNodeBase & {
	type: "MemberExpression";
	object: AnyExpression | Super;
	property: StaticMemberProperty | ComputedMemberProperty;
};

export const memberExpression = createBuilder<MemberExpression>(
	"MemberExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			object: true,
			property: true,
		},
	},
);
