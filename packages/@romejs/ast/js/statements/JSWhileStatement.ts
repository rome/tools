/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, AnyJSStatement, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../utils";

export type JSWhileStatement = JSNodeBase & {
	type: "JSWhileStatement";
	test: AnyJSExpression;
	body: AnyJSStatement;
};

export const jsWhileStatement = createBuilder<JSWhileStatement>(
	"JSWhileStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			test: true,
			body: true,
		},
	},
);
