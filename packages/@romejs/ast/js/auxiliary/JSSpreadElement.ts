/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, JSNodeBase} from "@romejs/ast";
import {createQuickBuilder} from "../../utils";

export type JSSpreadElement = JSNodeBase & {
	type: "JSSpreadElement";
	argument: AnyJSExpression;
};

export const jsSpreadElement = createQuickBuilder<JSSpreadElement, "argument">(
	"JSSpreadElement",
	"argument",
	{
		bindingKeys: {},
		visitorKeys: {
			argument: true,
		},
	},
);
