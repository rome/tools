/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {FunctionHead, JSNodeBase} from "../index";
import {createBuilder} from "../utils";
import {BlockStatement} from "../statements/BlockStatement";
import {AnyExpression} from "../unions";

export type ArrowFunctionExpression = JSNodeBase & {
	type: "ArrowFunctionExpression";
	head: FunctionHead;
	body: BlockStatement | AnyExpression;
	generator?: void;
};

export const arrowFunctionExpression = createBuilder<ArrowFunctionExpression>(
	"ArrowFunctionExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			head: true,
			body: true,
		},
	},
);
