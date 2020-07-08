/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSRegExpEscapedCharacter,
	JSRegExpCharSetRange,
	NodeBaseWithComments,
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type JSRegExpCharSet = NodeBaseWithComments & {
	type: "JSRegExpCharSet";
	invert?: boolean;
	body: Array<JSRegExpCharSetRange | AnyJSRegExpEscapedCharacter>;
};

export const jsRegExpCharSet = createBuilder<JSRegExpCharSet>(
	"JSRegExpCharSet",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
		},
	},
);
