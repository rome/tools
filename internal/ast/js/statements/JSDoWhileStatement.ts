/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	AnyJSStatement,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSDoWhileStatement extends NodeBaseWithComments {
	readonly type: "JSDoWhileStatement";
	readonly body: AnyJSStatement;
	readonly test: AnyJSExpression;
}

export const jsDoWhileStatement = createBuilder<JSDoWhileStatement>(
	"JSDoWhileStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			test: true,
			body: true,
		},
	},
);
