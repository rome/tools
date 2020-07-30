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

export interface JSWhileStatement extends NodeBaseWithComments {
	readonly type: "JSWhileStatement";
	readonly test: AnyJSExpression;
	readonly body: AnyJSStatement;
}

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
