/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSIdentifier,
	JSImportSpecifierLocal,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSImportSpecifier extends NodeBaseWithComments {
	readonly type: "JSImportSpecifier";
	readonly imported: JSIdentifier;
	readonly local: JSImportSpecifierLocal;
}

export const jsImportSpecifier = createBuilder<JSImportSpecifier>(
	"JSImportSpecifier",
	{
		bindingKeys: {
			local: true,
		},
		visitorKeys: {
			imported: true,
			local: true,
		},
	},
);
