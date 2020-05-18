/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, JSNodeBase} from "@romejs/ast";
import {createQuickBuilder} from "../utils";

export type JSComputedMemberProperty = JSNodeBase & {
	type: "JSComputedMemberProperty";
	value: AnyJSExpression;
	optional?: boolean;
};

export const jsComputedMemberProperty = createQuickBuilder<
	JSComputedMemberProperty,
	"value"
>(
	"JSComputedMemberProperty",
	"value",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
