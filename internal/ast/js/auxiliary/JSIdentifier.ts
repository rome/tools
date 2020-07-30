/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSIdentifier extends NodeBaseWithComments {
	readonly type: "JSIdentifier";
	readonly name: string;
	readonly definite?: boolean;
}

export const jsIdentifier = createQuickBuilder<JSIdentifier, "name">(
	"JSIdentifier",
	"name",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
