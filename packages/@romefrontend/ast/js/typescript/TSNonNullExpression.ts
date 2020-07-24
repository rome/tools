/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface TSNonNullExpression extends NodeBaseWithComments {
	type: "TSNonNullExpression";
	expression: AnyJSExpression;
}

export const tsNonNullExpression = createBuilder<TSNonNullExpression>(
	"TSNonNullExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			expression: true,
		},
	},
);
