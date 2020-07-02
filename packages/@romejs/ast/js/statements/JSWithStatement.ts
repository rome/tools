/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, AnyJSStatement, NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSWithStatement = NodeBaseWithComments & {
	type: "JSWithStatement";
	object: AnyJSExpression;
	body: AnyJSStatement;
};

export const jsWithStatement = createBuilder<JSWithStatement>(
	"JSWithStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			object: true,
			body: true,
		},
	},
);
