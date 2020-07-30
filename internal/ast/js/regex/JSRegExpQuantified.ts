/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSRegExpBodyItem, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSRegExpQuantified extends NodeBaseWithComments {
	readonly type: "JSRegExpQuantified";
	readonly target: AnyJSRegExpBodyItem;
	readonly lazy?: boolean;
	readonly min: number;
	readonly max?: number;
}

export const jsRegExpQuantified = createBuilder<JSRegExpQuantified>(
	"JSRegExpQuantified",
	{
		bindingKeys: {},
		visitorKeys: {
			target: true,
		},
	},
);
