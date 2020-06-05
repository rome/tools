/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSRegExpExpression, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSRegExpGroupCapture = JSNodeBase & {
	type: "JSRegExpGroupCapture";
	expression: AnyJSRegExpExpression;
	name?: string;
};

export const jsRegExpGroupCapture = createBuilder<JSRegExpGroupCapture>(
	"JSRegExpGroupCapture",
	{
		bindingKeys: {},
		visitorKeys: {
			expression: true,
		},
	},
);
