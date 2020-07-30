/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSRegExpNumericBackReference extends NodeBaseWithComments {
	readonly type: "JSRegExpNumericBackReference";
	readonly value: number;
}

export const jsRegExpNumericBackReference = createBuilder<JSRegExpNumericBackReference>(
	"JSRegExpNumericBackReference",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
