/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSRegExpCharacter = NodeBaseWithComments & {
	type: "JSRegExpCharacter";
	value: string;
};

export const jsRegExpCharacter = createBuilder<JSRegExpCharacter>(
	"JSRegExpCharacter",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
