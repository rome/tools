/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSTargetAssignmentPattern,
	AnyTSPrimary,
	NodeBaseWithComments,
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface TSAssignmentAsExpression extends NodeBaseWithComments {
	type: "TSAssignmentAsExpression";
	typeAnnotation: AnyTSPrimary;
	expression: AnyJSTargetAssignmentPattern;
}

export const tsAssignmentAsExpression = createBuilder<TSAssignmentAsExpression>(
	"TSAssignmentAsExpression",
	{
		bindingKeys: {},
		visitorKeys: {expression: true, typeAnnotation: true},
	},
);
