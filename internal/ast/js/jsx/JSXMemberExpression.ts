/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSXIdentifier,
	JSXNamespacedName,
	JSXReferenceIdentifier,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSXMemberExpression extends NodeBaseWithComments {
	readonly type: "JSXMemberExpression";
	object:
		| JSXMemberExpression
		| JSXIdentifier
		| JSXReferenceIdentifier
		| JSXNamespacedName;
	readonly property: JSXIdentifier;
}

export const jsxMemberExpression = createBuilder<JSXMemberExpression>(
	"JSXMemberExpression",
	{
		bindingKeys: {},
		visitorKeys: {
			object: true,
			property: true,
		},
	},
);
