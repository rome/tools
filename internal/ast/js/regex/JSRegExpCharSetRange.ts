/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSRegExpEscapedCharacter, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSRegExpCharSetRange extends NodeBaseWithComments {
	readonly type: "JSRegExpCharSetRange";
	readonly start: AnyJSRegExpEscapedCharacter;
	readonly end: AnyJSRegExpEscapedCharacter;
}

export const jsRegExpCharSetRange = createBuilder<JSRegExpCharSetRange>(
	"JSRegExpCharSetRange",
	{
		bindingKeys: {},
		visitorKeys: {
			start: true,
			end: true,
		},
	},
);
