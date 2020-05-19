/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, TSTypeParameter} from "@romejs/ast";
import {createBuilder} from "../utils";

export type TSTypeParameterDeclaration = JSNodeBase & {
	type: "TSTypeParameterDeclaration";
	params: Array<TSTypeParameter>;
};

export const tsTypeParameterDeclaration = createBuilder<TSTypeParameterDeclaration>(
	"TSTypeParameterDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {
			params: true,
		},
	},
);
