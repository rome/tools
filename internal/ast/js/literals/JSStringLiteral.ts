/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSStringLiteral extends NodeBaseWithComments {
	readonly type: "JSStringLiteral";
	readonly value: string;
}

export const jsStringLiteral = createQuickBuilder<JSStringLiteral, "value">(
	"JSStringLiteral",
	"value",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
