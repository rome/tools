/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, NodeBaseWithComments} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSPatternMeta extends NodeBaseWithComments {
	readonly type: "JSPatternMeta";
	readonly typeAnnotation?: AnyTSPrimary;
	readonly optional?: boolean;
	readonly accessibility?: string;
	readonly definite?: boolean;
	readonly readonly?: boolean;
}

export const jsPatternMeta = createQuickBuilder<JSPatternMeta, "typeAnnotation">(
	"JSPatternMeta",
	"typeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {
			typeAnnotation: true,
		},
	},
);
