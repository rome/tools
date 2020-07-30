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
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSPropertySignature extends NodeBaseWithComments {
	readonly type: "TSPropertySignature";
	readonly key: AnyJSObjectPropertyKey;
	readonly optional?: boolean;
	readonly readonly?: boolean;
	readonly typeAnnotation?: AnyTSPrimary;
}

export const tsPropertySignature = createBuilder<TSPropertySignature>(
	"TSPropertySignature",
	{
		bindingKeys: {},
		visitorKeys: {
			key: true,
			typeAnnotation: true,
		},
	},
);
