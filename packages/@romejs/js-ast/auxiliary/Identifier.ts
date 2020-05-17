/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from "../index";
import {createQuickBuilder} from "../utils";

export type Identifier = JSNodeBase & {
	type: "Identifier";
	name: string;
	definite?: boolean;
};

export const identifier = createQuickBuilder<Identifier, "name">(
	"Identifier",
	"name",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
