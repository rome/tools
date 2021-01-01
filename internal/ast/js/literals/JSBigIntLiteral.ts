/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSBigIntLiteral extends NodeBaseWithComments {
	readonly type: "JSBigIntLiteral";
	readonly value: string;
}

export const jsBigIntLiteral = createQuickBuilder<JSBigIntLiteral, "value">(
	"JSBigIntLiteral",
	"value",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
