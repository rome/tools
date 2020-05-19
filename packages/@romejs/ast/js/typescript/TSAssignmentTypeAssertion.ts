/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSTargetAssignmentPattern,
	AnyTSPrimary,
	JSNodeBase,
} from "@romejs/ast";
import {createBuilder} from "../utils";

export type TSAssignmentTypeAssertion = JSNodeBase & {
	type: "TSAssignmentTypeAssertion";
	typeAnnotation: AnyTSPrimary;
	expression: AnyJSTargetAssignmentPattern;
};

export const tsAssignmentTypeAssertion = createBuilder<TSAssignmentTypeAssertion>(
	"TSAssignmentTypeAssertion",
	{
		bindingKeys: {},
		visitorKeys: {expression: true, typeAnnotation: true},
	},
);
