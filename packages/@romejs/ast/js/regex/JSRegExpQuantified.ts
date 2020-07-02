/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSRegExpBodyItem, NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSRegExpQuantified = NodeBaseWithComments & {
	type: "JSRegExpQuantified";
	target: AnyJSRegExpBodyItem;
	lazy?: boolean;
	min: number;
	max?: number;
};

export const jsRegExpQuantified = createBuilder<JSRegExpQuantified>(
	"JSRegExpQuantified",
	{
		bindingKeys: {},
		visitorKeys: {
			target: true,
		},
	},
);
