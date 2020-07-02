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
	NodeBaseWithComments,
	JSVariableDeclaration,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSForInStatement = NodeBaseWithComments & {
	type: "JSForInStatement";
	left: JSVariableDeclaration | AnyJSTargetAssignmentPattern;
	right: AnyJSExpression;
	body: AnyJSStatement;
};

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
