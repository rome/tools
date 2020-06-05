/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, JSPatternMeta} from "@romejs/ast";
import {createQuickBuilder} from "../../utils";

export type JSBindingIdentifier = JSNodeBase & {
	type: "JSBindingIdentifier";
	name: string;
	definite?: boolean;
	meta?: JSPatternMeta;
};

export const jsBindingIdentifier = createQuickBuilder<
	JSBindingIdentifier,
	"name"
>(
	"JSBindingIdentifier",
	"name",
	{
		bindingKeys: {},
		visitorKeys: {
			meta: true,
		},
	},
);
