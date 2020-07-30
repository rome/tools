/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	AnyJSTargetBindingPattern,
	JSPatternMeta,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSBindingAssignmentPattern extends NodeBaseWithComments {
	readonly type: "JSBindingAssignmentPattern";
	readonly left: AnyJSTargetBindingPattern;
	readonly right: AnyJSExpression;
	readonly meta?: JSPatternMeta;
}

export const jsBindingAssignmentPattern = createBuilder<JSBindingAssignmentPattern>(
	"JSBindingAssignmentPattern",
	{
		bindingKeys: {
			left: true,
		},
		visitorKeys: {
			left: true,
			right: true,
			meta: true,
		},
	},
);
