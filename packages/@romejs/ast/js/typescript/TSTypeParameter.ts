/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSTypeParameter = JSNodeBase & {
	type: "TSTypeParameter";
	name: string;
	default?: AnyTSPrimary;
	constraint?: AnyTSPrimary;
};

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
