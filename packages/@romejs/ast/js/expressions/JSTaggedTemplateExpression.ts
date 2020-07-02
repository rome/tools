/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	NodeBaseWithComments,
	JSTemplateLiteral,
	TSTypeParameterInstantiation,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSTaggedTemplateExpression = NodeBaseWithComments & {
	type: "JSTaggedTemplateExpression";
	tag: AnyJSExpression;
	quasi: JSTemplateLiteral;
	typeArguments?: TSTypeParameterInstantiation;
};

export const jsTaggedTemplateExpression = createBuilder<JSTaggedTemplateExpression>(
	"JSTaggedTemplateExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			tag: true,
			quasi: true,
			typeArguments: true,
		},
	},
);
