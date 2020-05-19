/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSIdentifier, JSImportSpecifierLocal, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../utils";

export type JSImportSpecifier = JSNodeBase & {
	type: "JSImportSpecifier";
	imported: JSIdentifier;
	local: JSImportSpecifierLocal;
};

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
