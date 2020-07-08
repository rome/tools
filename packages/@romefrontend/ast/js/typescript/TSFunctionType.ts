/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyTSPrimary,
	NodeBaseWithComments,
	TSSignatureDeclarationMeta,
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type TSFunctionType = NodeBaseWithComments & {
	type: "TSFunctionType";
	meta: TSSignatureDeclarationMeta;
	typeAnnotation: AnyTSPrimary;
};

export const tsFunctionType = createBuilder<TSFunctionType>(
	"TSFunctionType",
	{
		bindingKeys: {},
		visitorKeys: {
			meta: true,
			typeAnnotation: true,
		},
	},
);
