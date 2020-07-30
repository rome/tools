/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSPatternMeta, NodeBaseWithComments} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSReferenceIdentifier extends NodeBaseWithComments {
	readonly type: "JSReferenceIdentifier";
	readonly name: string;
	readonly definite?: boolean;
	readonly meta?: JSPatternMeta;
}

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
