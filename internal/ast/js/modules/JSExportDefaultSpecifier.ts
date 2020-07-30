/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSIdentifier, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSExportDefaultSpecifier extends NodeBaseWithComments {
	readonly type: "JSExportDefaultSpecifier";
	readonly exported: JSIdentifier;
}

export const jsExportDefaultSpecifier = createBuilder<JSExportDefaultSpecifier>(
	"JSExportDefaultSpecifier",
	{
		bindingKeys: {},
		visitorKeys: {
			exported: true,
		},
	},
);
