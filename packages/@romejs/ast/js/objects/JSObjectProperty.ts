/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	JSComputedPropertyKey,
	JSStaticPropertyKey,
	NodeBaseWithComments,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSObjectProperty = NodeBaseWithComments & {
	type: "JSObjectProperty";
	key: JSStaticPropertyKey | JSComputedPropertyKey;
	value: AnyJSExpression;
};

export const jsObjectProperty = createBuilder<JSObjectProperty>(
	"JSObjectProperty",
	{
		bindingKeys: {},
		visitorKeys: {
			key: true,
			value: true,
		},
	},
);
