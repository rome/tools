/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@romefrontend/ast";
import {createQuickBuilder} from "../../utils";

export type JSIdentifier = NodeBaseWithComments & {
	type: "JSIdentifier";
	name: string;
	definite?: boolean;
};

export const jsIdentifier = createQuickBuilder<JSIdentifier, "name">(
	"JSIdentifier",
	"name",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
