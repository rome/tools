/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSRegExpExpression, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSRegExpGroupCapture extends NodeBaseWithComments {
	readonly type: "JSRegExpGroupCapture";
	readonly expression: AnyJSRegExpExpression;
	readonly name?: string;
}

export const jsRegExpGroupCapture = createBuilder<JSRegExpGroupCapture>(
	"JSRegExpGroupCapture",
	{
		bindingKeys: {},
		visitorKeys: {
			expression: true,
		},
	},
);
