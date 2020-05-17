/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyExpression,
	JSNodeBase,
	TSTypeParameterInstantiation,
	TemplateLiteral,
} from "../index";
import {createBuilder} from "../utils";

export type TaggedTemplateExpression = JSNodeBase & {
	type: "TaggedTemplateExpression";
	tag: AnyExpression;
	quasi: TemplateLiteral;
	typeArguments?: TSTypeParameterInstantiation;
};

export const taggedTemplateExpression = createBuilder<TaggedTemplateExpression>(
	"TaggedTemplateExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			tag: true,
			quasi: true,
			typeArguments: true,
		},
	},
);
