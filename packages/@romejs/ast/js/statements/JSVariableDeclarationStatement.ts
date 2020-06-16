/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, JSVariableDeclaration} from "@romejs/ast";
import {createQuickBuilder} from "../../utils";

export type JSVariableDeclarationStatement = JSNodeBase & {
	type: "JSVariableDeclarationStatement";
	declaration: JSVariableDeclaration;
	declare?: boolean;
};

export const jsVariableDeclarationStatement = createQuickBuilder<
	JSVariableDeclarationStatement,
	"declaration"
>(
	"JSVariableDeclarationStatement",
	"declaration",
	{
		bindingKeys: {
			declaration: true,
		},
		visitorKeys: {
			declaration: true,
		},
	},
);
