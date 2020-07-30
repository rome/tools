/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSThrowStatement extends NodeBaseWithComments {
	readonly type: "JSThrowStatement";
	readonly argument: AnyJSExpression;
}

export const jsThrowStatement = createBuilder<JSThrowStatement>(
	"JSThrowStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			argument: true,
		},
	},
);
