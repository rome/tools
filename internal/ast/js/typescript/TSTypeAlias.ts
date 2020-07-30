/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyTSPrimary,
	JSBindingIdentifier,
	NodeBaseWithComments,
	TSTypeParameterDeclaration,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSTypeAlias extends NodeBaseWithComments {
	readonly type: "TSTypeAlias";
	readonly id: JSBindingIdentifier;
	readonly typeParameters?: TSTypeParameterDeclaration;
	readonly right: AnyTSPrimary;
	readonly declare?: boolean | undefined;
}

export const tsTypeAlias = createBuilder<TSTypeAlias>(
	"TSTypeAlias",
	{
		bindingKeys: {
			id: true,
		},
		visitorKeys: {
			id: true,
			typeParameters: true,
			right: true,
		},
	},
);
