/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSBinaryExpression extends NodeBaseWithComments {
	readonly type: "JSBinaryExpression";
	readonly operator: BinaryOperator;
	readonly left: AnyJSExpression;
	readonly right: AnyJSExpression;
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
