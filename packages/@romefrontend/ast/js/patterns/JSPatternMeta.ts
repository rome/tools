/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, NodeBaseWithComments} from "@romefrontend/ast";
import {createQuickBuilder} from "../../utils";

export type JSPatternMeta = NodeBaseWithComments & {
	type: "JSPatternMeta";
	typeAnnotation?: AnyTSPrimary;
	optional?: boolean;
	accessibility?: string;
	definite?: boolean;
	readonly?: boolean;
};

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
