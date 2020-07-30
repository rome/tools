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

export interface JSForInStatement extends NodeBaseWithComments {
	readonly type: "JSForInStatement";
	readonly left: JSVariableDeclaration | AnyJSTargetAssignmentPattern;
	readonly right: AnyJSExpression;
	readonly body: AnyJSStatement;
}

export const jsForInStatement = createBuilder<JSForInStatement>(
	"JSForInStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			left: true,
			right: true,
			body: true,
		},
	},
);
