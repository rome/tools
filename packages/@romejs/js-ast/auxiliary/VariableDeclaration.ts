/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, VariableDeclarator} from "../index";
import {createBuilder} from "../utils";

export type VariableDeclarationKind = "var" | "let" | "const";

export type VariableDeclaration = JSNodeBase & {
	type: "VariableDeclaration";
	kind: VariableDeclarationKind;
	declarations: Array<VariableDeclarator>;
};

export const variableDeclaration = createBuilder<VariableDeclaration>(
	"VariableDeclaration",
	{
		bindingKeys: {
			declarations: true,
		},
		visitorKeys: {
			declarations: true,
		},
	},
);
