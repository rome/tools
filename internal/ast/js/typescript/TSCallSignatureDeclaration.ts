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

export interface TSCallSignatureDeclaration extends NodeBaseWithComments {
	readonly type: "TSCallSignatureDeclaration";
	readonly meta: TSSignatureDeclarationMeta;
	readonly typeAnnotation?: AnyTSPrimary;
}

export const tsCallSignatureDeclaration = createBuilder<TSCallSignatureDeclaration>(
	"TSCallSignatureDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {
			meta: true,
			typeAnnotation: true,
		},
	},
);
