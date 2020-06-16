/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyTSPrimary,
	JSNodeBase,
	TSSignatureDeclarationMeta,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSConstructSignatureDeclaration = JSNodeBase & {
	type: "TSConstructSignatureDeclaration";
	meta: TSSignatureDeclarationMeta;
	typeAnnotation?: AnyTSPrimary;
};

export const tsConstructSignatureDeclaration = createBuilder<TSConstructSignatureDeclaration>(
	"TSConstructSignatureDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {
			meta: true,
			typeAnnotation: true,
		},
	},
);
