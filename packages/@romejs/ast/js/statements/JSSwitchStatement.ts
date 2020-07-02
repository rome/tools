/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, NodeBaseWithComments, JSSwitchCase} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSSwitchStatement = NodeBaseWithComments & {
	type: "JSSwitchStatement";
	discriminant: AnyJSExpression;
	cases: Array<JSSwitchCase>;
};

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
