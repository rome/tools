/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, NodeBaseWithComments} from "@romejs/ast";
import {createQuickBuilder} from "../../utils";

export type JSComputedPropertyKey = NodeBaseWithComments & {
	type: "JSComputedPropertyKey";
	value: AnyJSExpression;
};

export const jsComputedPropertyKey = createQuickBuilder<
	JSComputedPropertyKey,
	"value"
>(
	"JSComputedPropertyKey",
	"value",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
