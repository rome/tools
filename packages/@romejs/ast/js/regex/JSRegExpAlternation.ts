/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSRegExpExpression,
	NodeBaseWithComments,
	JSRegExpSubExpression,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSRegExpAlternation = NodeBaseWithComments & {
	type: "JSRegExpAlternation";
	left: AnyJSRegExpExpression;
	right: JSRegExpSubExpression;
};

export const jsRegExpAlternation = createBuilder<JSRegExpAlternation>(
	"JSRegExpAlternation",
	{
		bindingKeys: {},
		visitorKeys: {
			left: true,
			right: true,
		},
	},
);
