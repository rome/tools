/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSRegExpExpression,
	JSRegExpSubExpression,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSRegExpAlternation extends NodeBaseWithComments {
	readonly type: "JSRegExpAlternation";
	readonly left: AnyJSRegExpExpression;
	readonly right: JSRegExpSubExpression;
}

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
