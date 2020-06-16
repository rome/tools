/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSBindingIdentifier,
	JSBindingObjectPatternProperty,
	JSNodeBase,
	JSPatternMeta,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSBindingObjectPattern = JSNodeBase & {
	meta?: JSPatternMeta;
	type: "JSBindingObjectPattern";
	properties: Array<JSBindingObjectPatternProperty>;
	rest: undefined | JSBindingIdentifier;
};

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
