/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	JSSwitchCase,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSSwitchStatement extends NodeBaseWithComments {
	readonly type: "JSSwitchStatement";
	readonly discriminant: AnyJSExpression;
	readonly cases: Array<JSSwitchCase>;
}

export const jsSwitchStatement = createBuilder<JSSwitchStatement>(
	"JSSwitchStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			discriminant: true,
			cases: true,
		},
	},
);
