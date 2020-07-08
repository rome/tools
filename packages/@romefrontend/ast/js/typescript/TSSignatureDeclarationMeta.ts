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
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type TSSignatureDeclarationMeta = NodeBaseWithComments & {
	type: "TSSignatureDeclarationMeta";
	parameters: Array<
		JSBindingIdentifier | JSBindingObjectPattern | JSBindingArrayPattern
	>;
	rest: undefined | AnyJSTargetBindingPattern;
	typeParameters: undefined | TSTypeParameterDeclaration;
};

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
