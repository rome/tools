/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSNumericLiteral extends NodeBaseWithComments {
	readonly type: "JSNumericLiteral";
	readonly value: number;
	readonly format?: "octal" | "binary" | "hex";
}

export const jsNumericLiteral = createQuickBuilder<JSNumericLiteral, "value">(
	"JSNumericLiteral",
	"value",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
