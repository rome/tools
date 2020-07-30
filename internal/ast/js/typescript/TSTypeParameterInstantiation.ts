/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSTypeParameterInstantiation extends NodeBaseWithComments {
	readonly type: "TSTypeParameterInstantiation";
	readonly params: Array<AnyTSPrimary>;
}

export const tsTypeParameterInstantiation = createBuilder<TSTypeParameterInstantiation>(
	"TSTypeParameterInstantiation",
	{
		bindingKeys: {},
		visitorKeys: {
			params: true,
		},
	},
);
