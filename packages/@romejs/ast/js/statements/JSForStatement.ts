/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	AnyJSStatement,
	JSNodeBase,
	JSVariableDeclaration,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSForStatement = JSNodeBase & {
	type: "JSForStatement";
	init?: JSVariableDeclaration | AnyJSExpression;
	test?: AnyJSExpression;
	update?: AnyJSExpression;
	body: AnyJSStatement;
};

export const jsForStatement = createBuilder<JSForStatement>(
	"JSForStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			init: true,
			test: true,
			update: true,
			body: true,
		},
	},
);
