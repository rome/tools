/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ConstJSExportModuleKind,
	JSStringLiteral,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSExportAllDeclaration extends NodeBaseWithComments {
	readonly type: "JSExportAllDeclaration";
	readonly source: JSStringLiteral;
	readonly exportKind?: ConstJSExportModuleKind;
	readonly declare?: boolean;
}

export const jsExportAllDeclaration = createBuilder<JSExportAllDeclaration>(
	"JSExportAllDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {
			source: true,
		},
	},
);
