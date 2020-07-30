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
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSRegExpCharSet extends NodeBaseWithComments {
	readonly type: "JSRegExpCharSet";
	readonly invert?: boolean;
	readonly body: Array<JSRegExpCharSetRange | AnyJSRegExpEscapedCharacter>;
}

export const jsRegExpCharSet = createBuilder<JSRegExpCharSet>(
	"JSRegExpCharSet",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
		},
	},
);
