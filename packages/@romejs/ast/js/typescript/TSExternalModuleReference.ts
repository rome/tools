/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, JSStringLiteral} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSExternalModuleReference = JSNodeBase & {
	type: "TSExternalModuleReference";
	expression: JSStringLiteral;
};

export const tsExternalModuleReference = createBuilder<TSExternalModuleReference>(
	"TSExternalModuleReference",
	{
		bindingKeys: {},
		visitorKeys: {expression: true},
	},
);
