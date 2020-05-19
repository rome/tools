/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	JSComputedMemberProperty,
	JSNodeBase,
	JSStaticMemberProperty,
	JSSuper,
} from "@romejs/ast";
import {createBuilder} from "../utils";

export type JSMemberExpression = JSNodeBase & {
	type: "JSMemberExpression";
	object: AnyJSExpression | JSSuper;
	property: JSStaticMemberProperty | JSComputedMemberProperty;
};

export const jsMemberExpression = createBuilder<JSMemberExpression>(
	"JSMemberExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			object: true,
			property: true,
		},
	},
);
