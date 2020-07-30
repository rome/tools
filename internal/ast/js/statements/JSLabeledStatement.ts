/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSStatement,
	JSIdentifier,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSLabeledStatement extends NodeBaseWithComments {
	readonly type: "JSLabeledStatement";
	readonly label: JSIdentifier;
	readonly body: AnyJSStatement;
}

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
