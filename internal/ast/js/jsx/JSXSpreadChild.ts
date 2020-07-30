/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSXSpreadChild extends NodeBaseWithComments {
	readonly type: "JSXSpreadChild";
	readonly expression: AnyJSExpression;
}

export const jsxSpreadChild = createBuilder<JSXSpreadChild>(
	"JSXSpreadChild",
	{
		bindingKeys: {},
		visitorKeys: {
			expression: true,
		},
	},
);
