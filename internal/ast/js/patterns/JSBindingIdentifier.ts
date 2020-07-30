/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSPatternMeta, NodeBaseWithComments} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSBindingIdentifier extends NodeBaseWithComments {
	readonly type: "JSBindingIdentifier";
	readonly name: string;
	readonly definite?: boolean;
	readonly meta?: JSPatternMeta;
}

export const jsBindingIdentifier = createQuickBuilder<
	JSBindingIdentifier,
	"name"
>(
	"JSBindingIdentifier",
	"name",
	{
		bindingKeys: {},
		visitorKeys: {
			meta: true,
		},
	},
);
