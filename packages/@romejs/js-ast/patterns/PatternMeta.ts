/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, JSNodeBase} from "../index";
import {createQuickBuilder} from "../utils";

export type PatternMeta = JSNodeBase & {
	type: "PatternMeta";
	typeAnnotation?: AnyTSPrimary;
	optional?: boolean;
	accessibility?: string;
	definite?: boolean;
	readonly?: boolean;
};

export const patternMeta = createQuickBuilder<PatternMeta, "typeAnnotation">(
	"PatternMeta",
	"typeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {
			typeAnnotation: true,
		},
	},
);
