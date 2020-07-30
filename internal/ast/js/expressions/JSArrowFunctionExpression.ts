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
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSArrowFunctionExpression extends NodeBaseWithComments {
	readonly type: "JSArrowFunctionExpression";
	readonly head: JSFunctionHead;
	readonly body: JSBlockStatement | AnyJSExpression;
	readonly generator?: void;
}

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
