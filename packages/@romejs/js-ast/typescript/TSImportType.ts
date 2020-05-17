/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyExpression,
	AnyTSEntityName,
	JSNodeBase,
	TSTypeParameterInstantiation,
} from "../index";
import {createBuilder} from "../utils";

export type TSImportType = JSNodeBase & {
	type: "TSImportType";
	argument: AnyExpression;
	typeParameters?: TSTypeParameterInstantiation;
	qualifier?: AnyTSEntityName;
};

export const tsImportType = createBuilder<TSImportType>(
	"TSImportType",
	{
		bindingKeys: {},
		visitorKeys: {
			argument: true,
			typeParameters: true,
			qualifier: true,
		},
	},
);
