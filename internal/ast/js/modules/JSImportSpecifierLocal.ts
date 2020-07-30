/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSBindingIdentifier, NodeBaseWithComments} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSImportSpecifierLocal extends NodeBaseWithComments {
	readonly type: "JSImportSpecifierLocal";
	readonly name: JSBindingIdentifier;
}

export const jsImportSpecifierLocal = createQuickBuilder<
	JSImportSpecifierLocal,
	"name"
>(
	"JSImportSpecifierLocal",
	"name",
	{
		bindingKeys: {
			name: true,
		},
		visitorKeys: {
			name: true,
		},
	},
);
