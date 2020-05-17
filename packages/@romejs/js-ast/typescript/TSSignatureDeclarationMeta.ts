/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyTargetBindingPattern,
	BindingArrayPattern,
	BindingIdentifier,
	BindingObjectPattern,
	JSNodeBase,
	TSTypeParameterDeclaration,
} from "../index";
import {createBuilder} from "../utils";

export type TSSignatureDeclarationMeta = JSNodeBase & {
	type: "TSSignatureDeclarationMeta";
	parameters: Array<
		BindingIdentifier | BindingObjectPattern | BindingArrayPattern
	>;
	rest: undefined | AnyTargetBindingPattern;
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
