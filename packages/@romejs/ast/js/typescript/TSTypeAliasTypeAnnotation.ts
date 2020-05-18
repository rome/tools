/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyTSPrimary,
	JSBindingIdentifier,
	JSNodeBase,
	TSTypeParameterDeclaration,
} from "@romejs/ast";
import {createBuilder} from "../utils";

export type TSTypeAliasTypeAnnotation = JSNodeBase & {
	type: "TSTypeAliasTypeAnnotation";
	id: JSBindingIdentifier;
	typeParameters?: TSTypeParameterDeclaration;
	right: AnyTSPrimary;
	declare?: boolean | undefined;
};

export const tsTypeAliasTypeAnnotation = createBuilder<TSTypeAliasTypeAnnotation>(
	"TSTypeAliasTypeAnnotation",
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
