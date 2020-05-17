/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyExpression, JSNodeBase} from "../index";
import {createBuilder} from "../utils";

export type AwaitExpression = JSNodeBase & {
	type: "AwaitExpression";
	argument?: AnyExpression;
};

export const awaitExpression = createBuilder<AwaitExpression>(
	"AwaitExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			argument: true,
		},
	},
);
