/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments, TSTypeParameter} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSTypeParameterDeclaration extends NodeBaseWithComments {
	readonly type: "TSTypeParameterDeclaration";
	readonly params: Array<TSTypeParameter>;
}

export const tsTypeParameterDeclaration = createBuilder<TSTypeParameterDeclaration>(
	"TSTypeParameterDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {
			params: true,
		},
	},
);
