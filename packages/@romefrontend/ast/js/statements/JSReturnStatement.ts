/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, NodeBaseWithComments} from "@romefrontend/ast";
import {createQuickBuilder} from "../../utils";

export interface JSReturnStatement extends NodeBaseWithComments {
	type: "JSReturnStatement";
	argument?: AnyJSExpression;
}

export const jsReturnStatement = createQuickBuilder<
	JSReturnStatement,
	"argument"
>(
	"JSReturnStatement",
	"argument",
	{
		bindingKeys: {},
		visitorKeys: {
			argument: true,
		},
	},
);
