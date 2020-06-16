/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSUnaryExpression = JSNodeBase & {
	type: "JSUnaryExpression";
	operator: UnaryOperator;
	prefix?: boolean;
	argument: AnyJSExpression;
};

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
