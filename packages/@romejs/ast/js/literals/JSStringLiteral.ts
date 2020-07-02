/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@romejs/ast";
import {createQuickBuilder} from "../../utils";

export type JSStringLiteral = NodeBaseWithComments & {
	type: "JSStringLiteral";
	value: string;
};

export const jsStringLiteral = createQuickBuilder<JSStringLiteral, "value">(
	"JSStringLiteral",
	"value",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
