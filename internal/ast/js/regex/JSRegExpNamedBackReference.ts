/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSRegExpNamedBackReference extends NodeBaseWithComments {
	readonly type: "JSRegExpNamedBackReference";
	readonly name: string;
}

export const jsRegExpNamedBackReference = createBuilder<JSRegExpNamedBackReference>(
	"JSRegExpNamedBackReference",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
