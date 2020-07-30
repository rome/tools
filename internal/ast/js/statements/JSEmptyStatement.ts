/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSEmptyStatement extends NodeBaseWithComments {
	readonly type: "JSEmptyStatement";
}

export const jsEmptyStatement = createBuilder<JSEmptyStatement>(
	"JSEmptyStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
