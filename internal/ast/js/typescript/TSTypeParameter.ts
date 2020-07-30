/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSTypeParameter extends NodeBaseWithComments {
	readonly type: "TSTypeParameter";
	readonly name: string;
	readonly default?: AnyTSPrimary;
	readonly constraint?: AnyTSPrimary;
}

export const tsTypeParameter = createBuilder<TSTypeParameter>(
	"TSTypeParameter",
	{
		bindingKeys: {},
		visitorKeys: {
			default: true,
			constraint: true,
		},
	},
);
