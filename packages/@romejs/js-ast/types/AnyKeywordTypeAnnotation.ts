/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from "../index";
import {createBuilder} from "../utils";

export type AnyKeywordTypeAnnotation = JSNodeBase & {
	type: "AnyKeywordTypeAnnotation";
};

export const anyKeywordTypeAnnotation = createBuilder<AnyKeywordTypeAnnotation>(
	"AnyKeywordTypeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
