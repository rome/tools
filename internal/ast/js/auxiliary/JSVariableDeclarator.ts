/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	AnyJSTargetBindingPattern,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSVariableDeclarator extends NodeBaseWithComments {
	readonly type: "JSVariableDeclarator";
	readonly id: AnyJSTargetBindingPattern;
	readonly init?: AnyJSExpression;
}

export const jsVariableDeclarator = createBuilder<JSVariableDeclarator>(
	"JSVariableDeclarator",
	{
		bindingKeys: {
			id: true,
		},
		visitorKeys: {
			id: true,
			init: true,
		},
	},
);
