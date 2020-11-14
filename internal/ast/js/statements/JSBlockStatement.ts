/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSStatement, JSDirective, NodeBaseWithComments} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSBlockStatement extends NodeBaseWithComments {
	readonly type: "JSBlockStatement";
	readonly body: AnyJSStatement[];
	readonly directives?: JSDirective[];
}

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
