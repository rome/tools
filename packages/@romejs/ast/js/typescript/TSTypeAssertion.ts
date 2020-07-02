/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, AnyTSPrimary, NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSTypeAssertion = NodeBaseWithComments & {
	type: "TSTypeAssertion";
	expression: AnyJSExpression;
	typeAnnotation: AnyTSPrimary;
};

export const tsTypeAssertion = createBuilder<TSTypeAssertion>(
	"TSTypeAssertion",
	{
		bindingKeys: {},
		visitorKeys: {
			expression: true,
			typeAnnotation: true,
		},
	},
);
