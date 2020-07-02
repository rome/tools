/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ConstExportModuleKind, NodeBaseWithComments, JSStringLiteral} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSExportAllDeclaration = NodeBaseWithComments & {
	type: "JSExportAllDeclaration";
	source: JSStringLiteral;
	exportKind?: ConstExportModuleKind;
	declare?: boolean;
};

export const jsExportAllDeclaration = createBuilder<JSExportAllDeclaration>(
	"JSExportAllDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {
			source: true,
		},
	},
);
