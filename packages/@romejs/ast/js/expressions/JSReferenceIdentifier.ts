/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSPatternMeta, NodeBaseWithComments} from "@romejs/ast";
import {createQuickBuilder} from "../../utils";

export type JSReferenceIdentifier = NodeBaseWithComments & {
	type: "JSReferenceIdentifier";
	name: string;
	definite?: boolean;
	meta?: JSPatternMeta;
};

export const jsReferenceIdentifier = createQuickBuilder<
	JSReferenceIdentifier,
	"name"
>(
	"JSReferenceIdentifier",
	"name",
	{
		bindingKeys: {},
		visitorKeys: {
			meta: true,
		},
	},
);
