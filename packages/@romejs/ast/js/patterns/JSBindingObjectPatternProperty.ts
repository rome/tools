/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSBindingPattern,
	AnyJSObjectPropertyKey,
	NodeBaseWithComments,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSBindingObjectPatternProperty = NodeBaseWithComments & {
	type: "JSBindingObjectPatternProperty";
	key: AnyJSObjectPropertyKey;
	value: AnyJSBindingPattern;
	meta?: undefined;
};

export const jsBindingObjectPatternProperty = createBuilder<JSBindingObjectPatternProperty>(
	"JSBindingObjectPatternProperty",
	{
		bindingKeys: {
			value: true,
		},
		visitorKeys: {
			key: true,
			value: true,
		},
	},
);
