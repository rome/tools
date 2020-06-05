/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSOptionalType = JSNodeBase & {
	type: "TSOptionalType";
	typeAnnotation: AnyTSPrimary;
};

export const tsOptionalType = createBuilder<TSOptionalType>(
	"TSOptionalType",
	{
		bindingKeys: {},
		visitorKeys: {
			typeAnnotation: true,
		},
	},
);
