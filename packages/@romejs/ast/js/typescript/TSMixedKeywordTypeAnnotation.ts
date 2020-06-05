/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSMixedKeywordTypeAnnotation = JSNodeBase & {
	type: "TSMixedKeywordTypeAnnotation";
};

export const tsMixedKeywordTypeAnnotation = createBuilder<TSMixedKeywordTypeAnnotation>(
	"TSMixedKeywordTypeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
