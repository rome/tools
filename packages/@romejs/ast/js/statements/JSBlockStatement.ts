/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSStatement, JSDirective, NodeBaseWithComments} from "@romejs/ast";
import {createQuickBuilder} from "../../utils";

export type JSBlockStatement = NodeBaseWithComments & {
	type: "JSBlockStatement";
	body: Array<AnyJSStatement>;
	directives?: Array<JSDirective>;
};

export const jsBlockStatement = createQuickBuilder<JSBlockStatement, "body">(
	"JSBlockStatement",
	"body",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
			directives: true,
		},
	},
);
