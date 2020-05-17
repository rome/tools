/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BindingIdentifier,
	BindingObjectPatternProperty,
	JSNodeBase,
	PatternMeta,
} from "../index";
import {createBuilder} from "../utils";

export type BindingObjectPattern = JSNodeBase & {
	meta?: PatternMeta;
	type: "BindingObjectPattern";
	properties: Array<BindingObjectPatternProperty>;
	rest: undefined | BindingIdentifier;
};

export const bindingObjectPattern = createBuilder<BindingObjectPattern>(
	"BindingObjectPattern",
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
