/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyRegExpEscapedCharacter,
	JSNodeBase,
	RegExpCharSetRange,
} from "../index";
import {createBuilder} from "../utils";

export type RegExpCharSet = JSNodeBase & {
	type: "RegExpCharSet";
	invert?: boolean;
	body: Array<RegExpCharSetRange | AnyRegExpEscapedCharacter>;
};

export const regExpCharSet = createBuilder<RegExpCharSet>(
	"RegExpCharSet",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
		},
	},
);
