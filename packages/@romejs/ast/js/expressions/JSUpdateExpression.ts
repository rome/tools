/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSUpdateExpression = JSNodeBase & {
	type: "JSUpdateExpression";
	operator: UpdateOperator;
	argument: AnyJSExpression;
	prefix?: boolean;
};

export type UpdateOperator = "++" | "--";

export const jsUpdateExpression = createBuilder<JSUpdateExpression>(
	"JSUpdateExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			argument: true,
		},
	},
);
