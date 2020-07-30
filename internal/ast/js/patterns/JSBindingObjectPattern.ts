/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSBindingIdentifier,
	JSBindingObjectPatternProperty,
	JSPatternMeta,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSBindingObjectPattern extends NodeBaseWithComments {
	readonly meta?: JSPatternMeta;
	readonly type: "JSBindingObjectPattern";
	readonly properties: Array<JSBindingObjectPatternProperty>;
	readonly rest: undefined | JSBindingIdentifier;
}

export const jsBindingObjectPattern = createBuilder<JSBindingObjectPattern>(
	"JSBindingObjectPattern",
	{
		bindingKeys: {
			properties: true,
			rest: true,
		},
		visitorKeys: {
			properties: true,
			rest: true,
			meta: true,
		},
	},
);
