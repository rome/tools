/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSImportSpecifierLocal, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSImportDefaultSpecifier = JSNodeBase & {
	type: "JSImportDefaultSpecifier";
	local: JSImportSpecifierLocal;
};

export const jsImportDefaultSpecifier = createBuilder<JSImportDefaultSpecifier>(
	"JSImportDefaultSpecifier",
	{
		bindingKeys: {
			local: true,
		},
		visitorKeys: {
			local: true,
		},
	},
);
