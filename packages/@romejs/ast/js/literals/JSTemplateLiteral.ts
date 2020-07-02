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
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSTemplateLiteral = NodeBaseWithComments & {
	type: "JSTemplateLiteral";
	quasis: Array<JSTemplateElement>;
	expressions: Array<AnyJSExpression>;
};

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
