/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface JSBinaryExpression extends NodeBaseWithComments {
	type: "JSBinaryExpression";
	operator: BinaryOperator;
	left: AnyJSExpression;
	right: AnyJSExpression;
}

export type BinaryOperator =
	| "=="
	| "!="
	| "==="
	| "**"
	| "!=="
	| "<"
	| "<="
	| ">"
	| ">="
	| "<<"
	| ">>"
	| ">>>"
	| "+"
	| "-"
	| "*"
	| "/"
	| "%"
	| "|"
	| "^"
	| "&"
	| "in"
	| "instanceof";

export const jsBinaryExpression = createBuilder<JSBinaryExpression>(
	"JSBinaryExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			left: true,
			right: true,
		},
	},
);
