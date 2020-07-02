/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, AnyTSPrimary, NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSAsExpression = NodeBaseWithComments & {
	type: "TSAsExpression";
	typeAnnotation: AnyTSPrimary;
	expression: AnyJSExpression;
};

export const tsAsExpression = createBuilder<TSAsExpression>(
	"TSAsExpression",
	{
		bindingKeys: {},
		visitorKeys: {expression: true, typeAnnotation: true},
	},
);
