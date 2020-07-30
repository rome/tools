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
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSExportExternalSpecifier extends NodeBaseWithComments {
	readonly type: "JSExportExternalSpecifier";
	readonly exported: JSIdentifier;
	readonly local: JSIdentifier;
	readonly exportKind?: ConstJSExportModuleKind;
}

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
