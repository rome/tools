/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSTypeParameterInstantiation = JSNodeBase & {
	type: "TSTypeParameterInstantiation";
	params: Array<AnyTSPrimary>;
};

export const tsTypeParameterInstantiation = createBuilder<TSTypeParameterInstantiation>(
	"TSTypeParameterInstantiation",
	{
		bindingKeys: {},
		visitorKeys: {
			params: true,
		},
	},
);
