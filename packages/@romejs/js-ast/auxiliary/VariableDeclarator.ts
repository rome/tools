/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyExpression, AnyTargetBindingPattern, JSNodeBase} from "../index";
import {createBuilder} from "../utils";

export type VariableDeclarator = JSNodeBase & {
	type: "VariableDeclarator";
	id: AnyTargetBindingPattern;
	init?: AnyExpression;
};

export const variableDeclarator = createBuilder<VariableDeclarator>(
	"VariableDeclarator",
	{
		bindingKeys: {
			id: true,
		},
		visitorKeys: {
			id: true,
			init: true,
		},
	},
);
