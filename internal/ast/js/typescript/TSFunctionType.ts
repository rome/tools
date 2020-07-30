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
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSFunctionType extends NodeBaseWithComments {
	readonly type: "TSFunctionType";
	readonly meta: TSSignatureDeclarationMeta;
	readonly typeAnnotation: AnyTSPrimary;
}

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
