/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSIdentifier,
	JSNumericLiteral,
	JSPrivateName,
	JSStringLiteral,
	NodeBaseWithComments,
} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSStaticPropertyKey extends NodeBaseWithComments {
	readonly type: "JSStaticPropertyKey";
	readonly value:
		| JSIdentifier
		| JSPrivateName
		| JSStringLiteral
		| JSNumericLiteral;
}

export const jsStaticPropertyKey = createQuickBuilder<
	JSStaticPropertyKey,
	"value"
>(
	"JSStaticPropertyKey",
	"value",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
