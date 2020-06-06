/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSBindingIdentifier,
	JSNodeBase,
	JSStringLiteral,
	TSModuleBlock,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSModuleDeclaration = JSNodeBase & {
	type: "TSModuleDeclaration";
	id: JSBindingIdentifier | JSStringLiteral;
	global?: boolean;
	body?: TSModuleBlock | TSModuleDeclaration;
	declare?: boolean;
};

export const tsModuleDeclaration = createBuilder<TSModuleDeclaration>(
	"TSModuleDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {
			id: true,
			body: true,
		},
	},
);
