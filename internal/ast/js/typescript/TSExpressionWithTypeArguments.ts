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

export interface TSExpressionWithTypeArguments extends NodeBaseWithComments {
	readonly type: "TSExpressionWithTypeArguments";
	readonly expression: AnyTSEntityName;
	readonly typeParameters?: TSTypeParameterInstantiation;
}

export const tsExpressionWithTypeArguments = createBuilder<TSExpressionWithTypeArguments>(
	"TSExpressionWithTypeArguments",
	{
		bindingKeys: {},
		visitorKeys: {
			expression: true,
			typeParameters: true,
		},
	},
);
