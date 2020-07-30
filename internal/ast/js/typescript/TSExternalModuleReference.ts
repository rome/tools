/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSStringLiteral, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSExternalModuleReference extends NodeBaseWithComments {
	readonly type: "TSExternalModuleReference";
	readonly expression: JSStringLiteral;
}

export const tsExternalModuleReference = createBuilder<TSExternalModuleReference>(
	"TSExternalModuleReference",
	{
		bindingKeys: {},
		visitorKeys: {expression: true},
	},
);
