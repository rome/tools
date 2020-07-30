/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyTSPrimary,
	JSBindingIdentifier,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSIndexSignature extends NodeBaseWithComments {
	readonly type: "TSIndexSignature";
	readonly readonly?: boolean;
	readonly key: JSBindingIdentifier;
	readonly typeAnnotation: undefined | AnyTSPrimary;
}

export const tsIndexSignature = createBuilder<TSIndexSignature>(
	"TSIndexSignature",
	{
		bindingKeys: {
			key: true,
		},
		visitorKeys: {
			typeAnnotation: true,
			key: true,
		},
	},
);
