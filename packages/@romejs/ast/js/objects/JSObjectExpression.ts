/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, JSObjectProperties} from "@romejs/ast";
import {createQuickBuilder} from "../../utils";

export type JSObjectExpression = JSNodeBase & {
	type: "JSObjectExpression";
	properties: JSObjectProperties;
};

export const jsObjectExpression = createQuickBuilder<
	JSObjectExpression,
	"properties"
>(
	"JSObjectExpression",
	"properties",
	{
		bindingKeys: {},
		visitorKeys: {
			properties: true,
		},
	},
);
