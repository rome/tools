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
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSBindingObjectPatternProperty extends NodeBaseWithComments {
	readonly type: "JSBindingObjectPatternProperty";
	readonly key: AnyJSObjectPropertyKey;
	readonly value: AnyJSBindingPattern;
	readonly meta?: undefined;
}

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
