/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSBindingIdentifier,
	JSBlockStatement,
	JSFunctionHead,
	NodeBaseWithComments,
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface JSFunctionExpression extends NodeBaseWithComments {
	type: "JSFunctionExpression";
	id?: JSBindingIdentifier;
	head: JSFunctionHead;
	body: JSBlockStatement;
}

export const jsFunctionExpression = createBuilder<JSFunctionExpression>(
	"JSFunctionExpression",
	{
		bindingKeys: {
			id: true,
		},
		visitorKeys: {
			head: true,
			id: true,
			body: true,
		},
	},
);
