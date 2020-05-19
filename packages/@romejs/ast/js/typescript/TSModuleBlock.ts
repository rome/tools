/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSStatement, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../utils";

export type TSModuleBlock = JSNodeBase & {
	type: "TSModuleBlock";
	body: Array<AnyJSStatement>;
};

export const tsModuleBlock = createBuilder<TSModuleBlock>(
	"TSModuleBlock",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
		},
	},
);
