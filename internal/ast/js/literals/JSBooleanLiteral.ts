/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSBooleanLiteral extends NodeBaseWithComments {
	readonly type: "JSBooleanLiteral";
	readonly value: boolean;
}

export const jsBooleanLiteral = createQuickBuilder<JSBooleanLiteral, "value">(
	"JSBooleanLiteral",
	"value",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
