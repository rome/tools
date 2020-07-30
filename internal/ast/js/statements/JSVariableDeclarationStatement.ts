/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSVariableDeclaration, NodeBaseWithComments} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSVariableDeclarationStatement extends NodeBaseWithComments {
	readonly type: "JSVariableDeclarationStatement";
	readonly declaration: JSVariableDeclaration;
	readonly declare?: boolean;
}

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
