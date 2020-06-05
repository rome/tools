/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, AnyJSStatement, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSIfStatement = JSNodeBase & {
	type: "JSIfStatement";
	test: AnyJSExpression;
	consequent: AnyJSStatement;
	alternate?: AnyJSStatement;
};

export const jsIfStatement = createBuilder<JSIfStatement>(
	"JSIfStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			test: true,
			consequent: true,
			alternate: true,
		},
	},
);
