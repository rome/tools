/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSStringLiteral,
	JSXElement,
	JSXExpressionContainer,
	JSXFragment,
	JSXIdentifier,
	JSXNamespacedName,
	NodeBaseWithComments,
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type JSXAttribute = NodeBaseWithComments & {
	type: "JSXAttribute";
	name: JSXIdentifier | JSXNamespacedName;
	value?:
		| undefined
		| JSXElement
		| JSXFragment
		| JSStringLiteral
		| JSXExpressionContainer;
};

export const jsxAttribute = createBuilder<JSXAttribute>(
	"JSXAttribute",
	{
		bindingKeys: {},
		visitorKeys: {
			name: true,
			value: true,
		},
	},
);
