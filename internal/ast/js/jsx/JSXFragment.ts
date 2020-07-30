/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSXElement, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSXFragment extends NodeBaseWithComments {
	readonly type: "JSXFragment";
	readonly children: JSXElement["children"];
}

export const jsxFragment = createBuilder<JSXFragment>(
	"JSXFragment",
	{
		bindingKeys: {},
		visitorKeys: {
			children: true,
		},
	},
);
