/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSVariableDeclarator, NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type JSVariableDeclarationKind = "var" | "let" | "const";

export interface JSVariableDeclaration extends NodeBaseWithComments {
	type: "JSVariableDeclaration";
	kind: JSVariableDeclarationKind;
	declarations: Array<JSVariableDeclarator>;
}

export const jsVariableDeclaration = createBuilder<JSVariableDeclaration>(
	"JSVariableDeclaration",
	{
		bindingKeys: {
			declarations: true,
		},
		visitorKeys: {
			declarations: true,
		},
	},
);
