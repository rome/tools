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
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSAssignmentTypeAssertion extends NodeBaseWithComments {
	readonly type: "TSAssignmentTypeAssertion";
	readonly typeAnnotation: AnyTSPrimary;
	readonly expression: AnyJSTargetAssignmentPattern;
}

export const tsAssignmentTypeAssertion = createBuilder<TSAssignmentTypeAssertion>(
	"TSAssignmentTypeAssertion",
	{
		bindingKeys: {},
		visitorKeys: {expression: true, typeAnnotation: true},
	},
);
