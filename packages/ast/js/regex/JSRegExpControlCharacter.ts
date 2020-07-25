/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface JSRegExpControlCharacter extends NodeBaseWithComments {
	type: "JSRegExpControlCharacter";
}

export const jsRegExpControlCharacter = createBuilder<JSRegExpControlCharacter>(
	"JSRegExpControlCharacter",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
