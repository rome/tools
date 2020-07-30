/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSYieldExpression extends NodeBaseWithComments {
	readonly type: "JSYieldExpression";
	readonly delegate?: boolean;
	readonly argument?: AnyJSExpression;
}

export const jsYieldExpression = createBuilder<JSYieldExpression>(
	"JSYieldExpression",
	{bindingKeys: {}, visitorKeys: {argument: true}},
);
