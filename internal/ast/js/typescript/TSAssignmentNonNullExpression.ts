/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSTargetAssignmentPattern,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSAssignmentNonNullExpression extends NodeBaseWithComments {
	readonly type: "TSAssignmentNonNullExpression";
	readonly expression: AnyJSTargetAssignmentPattern;
}

export const tsAssignmentNonNullExpression = createBuilder<TSAssignmentNonNullExpression>(
	"TSAssignmentNonNullExpression",
	{
		bindingKeys: {},
		visitorKeys: {expression: true},
	},
);
