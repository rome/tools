/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	JSTemplateLiteral,
	NodeBaseWithComments,
	TSTypeParameterInstantiation,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSTaggedTemplateExpression extends NodeBaseWithComments {
	readonly type: "JSTaggedTemplateExpression";
	readonly tag: AnyJSExpression;
	readonly quasi: JSTemplateLiteral;
	readonly typeArguments?: TSTypeParameterInstantiation;
}

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
