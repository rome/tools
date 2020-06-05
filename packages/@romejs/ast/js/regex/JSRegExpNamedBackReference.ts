/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSRegExpNamedBackReference = JSNodeBase & {
	type: "JSRegExpNamedBackReference";
	name: string;
};

export const jsRegExpNamedBackReference = createBuilder<JSRegExpNamedBackReference>(
	"JSRegExpNamedBackReference",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
