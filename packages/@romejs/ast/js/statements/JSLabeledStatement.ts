/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSStatement, JSIdentifier, NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSLabeledStatement = NodeBaseWithComments & {
	type: "JSLabeledStatement";
	label: JSIdentifier;
	body: AnyJSStatement;
};

export const jsLabeledStatement = createBuilder<JSLabeledStatement>(
	"JSLabeledStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			label: true,
			body: true,
		},
	},
);
