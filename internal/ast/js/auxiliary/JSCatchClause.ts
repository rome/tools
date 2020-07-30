/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSBindingPattern,
	JSBlockStatement,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSCatchClause extends NodeBaseWithComments {
	readonly type: "JSCatchClause";
	readonly param?: AnyJSBindingPattern;
	readonly body: JSBlockStatement;
}

export const jsCatchClause = createBuilder<JSCatchClause>(
	"JSCatchClause",
	{
		bindingKeys: {
			param: true,
		},
		visitorKeys: {
			param: true,
			body: true,
		},
	},
);
