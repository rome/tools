/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../utils";

export type TSNumberKeywordTypeAnnotation = JSNodeBase & {
	type: "TSNumberKeywordTypeAnnotation";
};

export const tsNumberKeywordTypeAnnotation = createBuilder<TSNumberKeywordTypeAnnotation>(
	"TSNumberKeywordTypeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
