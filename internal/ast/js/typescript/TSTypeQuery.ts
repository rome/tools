/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyTSEntityName,
	NodeBaseWithComments,
	TSImportType,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSTypeQuery extends NodeBaseWithComments {
	readonly type: "TSTypeQuery";
	readonly exprName: TSImportType | AnyTSEntityName;
}

export const tsTypeQuery = createBuilder<TSTypeQuery>(
	"TSTypeQuery",
	{
		bindingKeys: {},
		visitorKeys: {
			exprName: true,
		},
	},
);
