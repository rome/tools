/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, JSBindingIdentifier, NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSIndexSignature = NodeBaseWithComments & {
	type: "TSIndexSignature";
	readonly?: boolean;
	key: JSBindingIdentifier;
	typeAnnotation: undefined | AnyTSPrimary;
};

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
