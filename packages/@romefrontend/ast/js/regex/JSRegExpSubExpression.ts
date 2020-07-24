/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSRegExpBodyItem, NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface JSRegExpSubExpression extends NodeBaseWithComments {
	type: "JSRegExpSubExpression";
	body: Array<AnyJSRegExpBodyItem>;
}

export const jsRegExpSubExpression = createBuilder<JSRegExpSubExpression>(
	"JSRegExpSubExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
		},
	},
);
