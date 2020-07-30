/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSStatement, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSModuleBlock extends NodeBaseWithComments {
	readonly type: "TSModuleBlock";
	readonly body: Array<AnyJSStatement>;
}

export const tsModuleBlock = createBuilder<TSModuleBlock>(
	"TSModuleBlock",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
		},
	},
);
