/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	AnyJSStatement,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSSwitchCase extends NodeBaseWithComments {
	readonly type: "JSSwitchCase";
	readonly test?: AnyJSExpression;
	readonly consequent: Array<AnyJSStatement>;
}

export const jsSwitchCase = createBuilder<JSSwitchCase>(
	"JSSwitchCase",
	{
		bindingKeys: {},
		visitorKeys: {
			test: true,
			consequent: true,
		},
	},
);
