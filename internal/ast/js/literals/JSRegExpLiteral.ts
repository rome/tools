/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSRegExpAlternation,
	JSRegExpSubExpression,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSRegExpLiteral extends NodeBaseWithComments {
	readonly type: "JSRegExpLiteral";
	readonly expression: JSRegExpSubExpression | JSRegExpAlternation;
	readonly global?: boolean;
	readonly multiline?: boolean;
	readonly sticky?: boolean;
	readonly insensitive?: boolean;
	readonly noDotNewline?: boolean;
	readonly unicode?: boolean;
}

export const jsRegExpLiteral = createBuilder<JSRegExpLiteral>(
	"JSRegExpLiteral",
	{
		bindingKeys: {},
		visitorKeys: {
			expression: true,
		},
	},
);
