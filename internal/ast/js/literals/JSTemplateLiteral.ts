/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	JSTemplateElement,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSTemplateLiteral extends NodeBaseWithComments {
	readonly type: "JSTemplateLiteral";
	readonly quasis: Array<JSTemplateElement>;
	readonly expressions: Array<AnyJSExpression>;
}

export const jsTemplateLiteral = createBuilder<JSTemplateLiteral>(
	"JSTemplateLiteral",
	{
		bindingKeys: {},
		visitorKeys: {
			quasis: true,
			expressions: true,
		},
	},
);
