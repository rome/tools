/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSIdentifier, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSExportNamespaceSpecifier extends NodeBaseWithComments {
	readonly type: "JSExportNamespaceSpecifier";
	readonly exported: JSIdentifier;
}

export const jsExportNamespaceSpecifier = createBuilder<JSExportNamespaceSpecifier>(
	"JSExportNamespaceSpecifier",
	{
		bindingKeys: {},
		visitorKeys: {
			exported: true,
		},
	},
);
