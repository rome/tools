/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSObjectPropertyKey,
	AnyTSPrimary,
	JSNodeBase,
	TSSignatureDeclarationMeta,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSMethodSignature = JSNodeBase & {
	key: AnyJSObjectPropertyKey;
	type: "TSMethodSignature";
	optional?: boolean;
	meta: TSSignatureDeclarationMeta;
	returnType?: AnyTSPrimary;
};

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
