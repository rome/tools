/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSNodeBase,
	JSRegExpAlternation,
	JSRegExpSubExpression,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSRegExpLiteral = JSNodeBase & {
	type: "JSRegExpLiteral";
	expression: JSRegExpSubExpression | JSRegExpAlternation;
	global?: boolean;
	multiline?: boolean;
	sticky?: boolean;
	insensitive?: boolean;
	noDotNewline?: boolean;
	unicode?: boolean;
};

export const jsRegExpLiteral = createBuilder<JSRegExpLiteral>(
	"JSRegExpLiteral",
	{
		bindingKeys: {},
		visitorKeys: {
			expression: true,
		},
	},
);
