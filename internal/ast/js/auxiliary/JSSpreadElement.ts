/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, NodeBaseWithComments} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSSpreadElement extends NodeBaseWithComments {
	readonly type: "JSSpreadElement";
	readonly argument: AnyJSExpression;
}

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
