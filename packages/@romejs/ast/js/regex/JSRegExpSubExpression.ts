/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSRegExpBodyItem, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../utils";

export type JSRegExpSubExpression = JSNodeBase & {
	type: "JSRegExpSubExpression";
	body: Array<AnyJSRegExpBodyItem>;
};

export const jsRegExpSubExpression = createBuilder<JSRegExpSubExpression>(
	"JSRegExpSubExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
		},
	},
);
