/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSTypeOperator = NodeBaseWithComments & {
	type: "TSTypeOperator";
	operator: "keyof" | "unique" | "readonly";
	typeAnnotation: AnyTSPrimary;
};

export const tsTypeOperator = createBuilder<TSTypeOperator>(
	"TSTypeOperator",
	{
		bindingKeys: {},
		visitorKeys: {
			typeAnnotation: true,
		},
	},
);
