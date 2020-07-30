/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	AnyJSStatement,
	AnyJSTargetAssignmentPattern,
	JSVariableDeclaration,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSForOfStatement extends NodeBaseWithComments {
	readonly type: "JSForOfStatement";
	readonly await?: boolean;
	readonly left: JSVariableDeclaration | AnyJSTargetAssignmentPattern;
	readonly right: AnyJSExpression;
	readonly body: AnyJSStatement;
}

export const jsForOfStatement = createBuilder<JSForOfStatement>(
	"JSForOfStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			left: true,
			right: true,
			body: true,
		},
	},
);
