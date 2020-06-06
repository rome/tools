/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	JSBlockStatement,
	JSFunctionHead,
	JSNodeBase,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSArrowFunctionExpression = JSNodeBase & {
	type: "JSArrowFunctionExpression";
	head: JSFunctionHead;
	body: JSBlockStatement | AnyJSExpression;
	generator?: void;
};

export const jsArrowFunctionExpression = createBuilder<JSArrowFunctionExpression>(
	"JSArrowFunctionExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			head: true,
			body: true,
		},
	},
);
