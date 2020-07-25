/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ConstJSExportModuleKind,
	JSIdentifier,
	JSReferenceIdentifier,
	NodeBaseWithComments,
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface JSExportLocalSpecifier extends NodeBaseWithComments {
	type: "JSExportLocalSpecifier";
	exported: JSIdentifier;
	local: JSReferenceIdentifier;
	exportKind?: ConstJSExportModuleKind;
}

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
