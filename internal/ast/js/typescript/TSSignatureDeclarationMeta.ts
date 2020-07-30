/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSTargetBindingPattern,
	JSBindingArrayPattern,
	JSBindingIdentifier,
	JSBindingObjectPattern,
	NodeBaseWithComments,
	TSTypeParameterDeclaration,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSSignatureDeclarationMeta extends NodeBaseWithComments {
	readonly type: "TSSignatureDeclarationMeta";
	readonly parameters: Array<
		JSBindingIdentifier | JSBindingObjectPattern | JSBindingArrayPattern
	>;
	readonly rest: undefined | AnyJSTargetBindingPattern;
	readonly typeParameters: undefined | TSTypeParameterDeclaration;
}

export const tsSignatureDeclarationMeta = createBuilder<TSSignatureDeclarationMeta>(
	"TSSignatureDeclarationMeta",
	{
		bindingKeys: {},
		visitorKeys: {
			parameters: true,
			rest: true,
			typeParameters: true,
		},
	},
);
