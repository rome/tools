/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSUnaryExpression extends NodeBaseWithComments {
	readonly type: "JSUnaryExpression";
	readonly operator: UnaryOperator;
	readonly prefix?: boolean;
	readonly argument: AnyJSExpression;
}

export type UnaryOperator =
	| "-"
	| "+"
	| "!"
	| "~"
	| "typeof"
	| "void"
	| "delete"
	| "throw";

export const jsUnaryExpression = createBuilder<JSUnaryExpression>(
	"JSUnaryExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			argument: true,
		},
	},
);
