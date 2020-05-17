/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, ObjectProperties} from "../index";
import {createQuickBuilder} from "../utils";

export type ObjectExpression = JSNodeBase & {
	type: "ObjectExpression";
	properties: ObjectProperties;
};

export const objectExpression = createQuickBuilder<
	ObjectExpression,
	"properties"
>(
	"ObjectExpression",
	"properties",
	{
		bindingKeys: {},
		visitorKeys: {
			properties: true,
		},
	},
);
