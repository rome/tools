/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments, TSTypeParameter} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSInferType = NodeBaseWithComments & {
	type: "TSInferType";
	typeParameter: TSTypeParameter;
};

export const tsInferType = createBuilder<TSInferType>(
	"TSInferType",
	{
		bindingKeys: {},
		visitorKeys: {
			typeParameter: true,
		},
	},
);
