/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSIdentifier,
	JSNodeBase,
	JSNumericLiteral,
	JSPrivateName,
	JSStringLiteral,
} from "@romejs/ast";
import {createQuickBuilder} from "../../utils";

export type JSStaticPropertyKey = JSNodeBase & {
	type: "JSStaticPropertyKey";
	value: JSIdentifier | JSPrivateName | JSStringLiteral | JSNumericLiteral;
};

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
