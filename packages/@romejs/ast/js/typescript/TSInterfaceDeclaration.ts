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
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSInterfaceDeclaration = NodeBaseWithComments & {
	type: "TSInterfaceDeclaration";
	id: JSBindingIdentifier;
	body: TSInterfaceBody;
	typeParameters?: TSTypeParameterDeclaration;
	extends?: Array<TSExpressionWithTypeArguments>;
	declare?: boolean;
};

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
