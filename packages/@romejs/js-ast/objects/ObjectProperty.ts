/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyExpression,
	ComputedPropertyKey,
	JSNodeBase,
	StaticPropertyKey,
} from "../index";
import {createBuilder} from "../utils";

export type ObjectProperty = JSNodeBase & {
	type: "ObjectProperty";
	key: StaticPropertyKey | ComputedPropertyKey;
	value: AnyExpression;
};

export const objectProperty = createBuilder<ObjectProperty>(
	"ObjectProperty",
	{
		bindingKeys: {},
		visitorKeys: {
			key: true,
			value: true,
		},
	},
);
