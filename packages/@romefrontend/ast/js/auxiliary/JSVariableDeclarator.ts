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
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type JSVariableDeclarator = NodeBaseWithComments & {
	type: "JSVariableDeclarator";
	id: AnyJSTargetBindingPattern;
	init?: AnyJSExpression;
};

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
