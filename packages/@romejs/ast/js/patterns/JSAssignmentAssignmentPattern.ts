/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	AnyJSTargetAssignmentPattern,
	JSNodeBase,
	JSPatternMeta,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSAssignmentAssignmentPattern = JSNodeBase & {
	type: "JSAssignmentAssignmentPattern";
	left: AnyJSTargetAssignmentPattern;
	right: AnyJSExpression;
	meta?: JSPatternMeta;
};

export const jsAssignmentAssignmentPattern = createBuilder<JSAssignmentAssignmentPattern>(
	"JSAssignmentAssignmentPattern",
	{
		bindingKeys: {},
		visitorKeys: {
			left: true,
			right: true,
			meta: true,
		},
	},
);
