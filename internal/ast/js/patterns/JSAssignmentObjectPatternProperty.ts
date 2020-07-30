/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSAssignmentPattern,
	AnyJSObjectPropertyKey,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSAssignmentObjectPatternProperty extends NodeBaseWithComments {
	readonly type: "JSAssignmentObjectPatternProperty";
	readonly key: AnyJSObjectPropertyKey;
	readonly value: AnyJSAssignmentPattern;
}

export const jsAssignmentObjectPatternProperty = createBuilder<JSAssignmentObjectPatternProperty>(
	"JSAssignmentObjectPatternProperty",
	{
		bindingKeys: {},
		visitorKeys: {
			key: true,
			value: true,
		},
	},
);
