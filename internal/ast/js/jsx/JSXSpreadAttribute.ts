/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSXSpreadAttribute extends NodeBaseWithComments {
	readonly type: "JSXSpreadAttribute";
	readonly argument: AnyJSExpression;
}

export const jsxSpreadAttribute = createBuilder<JSXSpreadAttribute>(
	"JSXSpreadAttribute",
	{
		bindingKeys: {},
		visitorKeys: {
			argument: true,
		},
	},
);
