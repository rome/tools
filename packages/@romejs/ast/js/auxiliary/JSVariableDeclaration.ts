/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, JSVariableDeclarator} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSVariableDeclarationKind = "var" | "let" | "const";

export type JSVariableDeclaration = JSNodeBase & {
	type: "JSVariableDeclaration";
	kind: JSVariableDeclarationKind;
	declarations: Array<JSVariableDeclarator>;
};

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
