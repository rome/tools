/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSBindingIdentifier,
	JSStringLiteral,
	NodeBaseWithComments,
	TSModuleBlock,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSModuleDeclaration extends NodeBaseWithComments {
	readonly type: "TSModuleDeclaration";
	readonly id: JSBindingIdentifier | JSStringLiteral;
	readonly global?: boolean;
	readonly body?: TSModuleBlock | TSModuleDeclaration;
	readonly declare?: boolean;
}

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
