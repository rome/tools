/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSXIdentifier extends NodeBaseWithComments {
	readonly type: "JSXIdentifier";
	readonly name: string;
}

export const jsxIdentifier = createQuickBuilder<JSXIdentifier, "name">(
	"JSXIdentifier",
	"name",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
