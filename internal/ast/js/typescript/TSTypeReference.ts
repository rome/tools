/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyTSEntityName,
	NodeBaseWithComments,
	TSTypeParameterInstantiation,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSTypeReference extends NodeBaseWithComments {
	readonly type: "TSTypeReference";
	readonly typeName: AnyTSEntityName;
	readonly typeParameters?: TSTypeParameterInstantiation;
}

export const tsTypeReference = createBuilder<TSTypeReference>(
	"TSTypeReference",
	{
		bindingKeys: {},
		visitorKeys: {
			typeName: true,
			typeParameters: true,
		},
	},
);
