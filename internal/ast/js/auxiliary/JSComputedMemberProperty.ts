/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, NodeBaseWithComments} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSComputedMemberProperty extends NodeBaseWithComments {
	readonly type: "JSComputedMemberProperty";
	readonly value: AnyJSExpression;
	readonly optional?: boolean;
}

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
