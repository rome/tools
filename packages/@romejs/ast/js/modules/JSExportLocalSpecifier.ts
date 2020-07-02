/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ConstExportModuleKind,
	JSIdentifier,
	NodeBaseWithComments,
	JSReferenceIdentifier,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSExportLocalSpecifier = NodeBaseWithComments & {
	type: "JSExportLocalSpecifier";
	exported: JSIdentifier;
	local: JSReferenceIdentifier;
	exportKind?: ConstExportModuleKind;
};

export const jsExportLocalSpecifier = createBuilder<JSExportLocalSpecifier>(
	"JSExportLocalSpecifier",
	{
		bindingKeys: {},
		visitorKeys: {
			local: true,
			exported: true,
		},
	},
);
