/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSObjectPropertyKey,
	AnyTSPrimary,
	NodeBaseWithComments,
	TSSignatureDeclarationMeta,
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface TSMethodSignature extends NodeBaseWithComments {
	key: AnyJSObjectPropertyKey;
	type: "TSMethodSignature";
	optional?: boolean;
	meta: TSSignatureDeclarationMeta;
	returnType?: AnyTSPrimary;
}

export const tsMethodSignature = createBuilder<TSMethodSignature>(
	"TSMethodSignature",
	{
		bindingKeys: {},
		visitorKeys: {
			key: true,
			meta: true,
			returnType: true,
		},
	},
);
