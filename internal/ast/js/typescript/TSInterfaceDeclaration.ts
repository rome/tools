/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSBindingIdentifier,
	NodeBaseWithComments,
	TSExpressionWithTypeArguments,
	TSInterfaceBody,
	TSTypeParameterDeclaration,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSInterfaceDeclaration extends NodeBaseWithComments {
	readonly type: "TSInterfaceDeclaration";
	readonly id: JSBindingIdentifier;
	readonly body: TSInterfaceBody;
	readonly typeParameters?: TSTypeParameterDeclaration;
	readonly extends?: Array<TSExpressionWithTypeArguments>;
	readonly declare?: boolean;
}

export const tsInterfaceDeclaration = createBuilder<TSInterfaceDeclaration>(
	"TSInterfaceDeclaration",
	{
		bindingKeys: {
			id: true,
		},
		visitorKeys: {
			id: true,
			body: true,
			typeParameters: true,
			extends: true,
		},
	},
);
