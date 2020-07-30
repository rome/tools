/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSConditionalExpression extends NodeBaseWithComments {
	readonly type: "JSConditionalExpression";
	readonly test: AnyJSExpression;
	readonly alternate: AnyJSExpression;
	readonly consequent: AnyJSExpression;
}

export const jsConditionalExpression = createBuilder<JSConditionalExpression>(
	"JSConditionalExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			test: true,
			consequent: true,
			alternate: true,
		},
	},
);
