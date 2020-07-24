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

export interface JSFunctionDeclaration extends NodeBaseWithComments {
	type: "JSFunctionDeclaration";
	id: JSBindingIdentifier;
	declare?: boolean;
	head: JSFunctionHead;
	body: JSBlockStatement;
}

export const jsFunctionDeclaration = createBuilder<JSFunctionDeclaration>(
	"JSFunctionDeclaration",
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
