/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSBlockStatement, NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSDoExpression = NodeBaseWithComments & {
	type: "JSDoExpression";
	body: JSBlockStatement;
};

export const jsDoExpression = createBuilder<JSDoExpression>(
	"JSDoExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
		},
	},
);
