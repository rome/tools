/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSAssignmentPattern,
	AnyJSExpression,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSAssignmentExpression extends NodeBaseWithComments {
	readonly type: "JSAssignmentExpression";
	readonly operator: AssignmentOperator;
	readonly left: AnyJSAssignmentPattern;
	readonly right: AnyJSExpression;
}

export type AssignmentOperator =
	| "="
	| "+="
	| "-="
	| "*="
	| "/="
	| "%="
	| "<<="
	| ">>="
	| ">>>="
	| "|="
	| "^="
	| "&="
	| "??=";

export const jsAssignmentExpression = createBuilder<JSAssignmentExpression>(
	"JSAssignmentExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			left: true,
			right: true,
		},
	},
);
