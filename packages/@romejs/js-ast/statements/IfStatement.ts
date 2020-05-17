/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyExpression, AnyStatement, JSNodeBase} from "../index";
import {createBuilder} from "../utils";

export type IfStatement = JSNodeBase & {
	type: "IfStatement";
	test: AnyExpression;
	consequent: AnyStatement;
	alternate?: AnyStatement;
};

export const ifStatement = createBuilder<IfStatement>(
	"IfStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			test: true,
			consequent: true,
			alternate: true,
		},
	},
);
