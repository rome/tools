/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ConstJSExportModuleKind,
	JSIdentifier,
	NodeBaseWithComments,
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type JSExportExternalSpecifier = NodeBaseWithComments & {
	type: "JSExportExternalSpecifier";
	exported: JSIdentifier;
	local: JSIdentifier;
	exportKind?: ConstJSExportModuleKind;
};

export const jsExportExternalSpecifier = createBuilder<JSExportExternalSpecifier>(
	"JSExportExternalSpecifier",
	{
		bindingKeys: {},
		visitorKeys: {
			exported: true,
			local: true,
		},
	},
);
