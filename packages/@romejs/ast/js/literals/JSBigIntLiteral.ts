/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSBigIntLiteral = JSNodeBase & {
	type: "JSBigIntLiteral";
	value: string;
};

export const jsBigIntLiteral = createBuilder<JSBigIntLiteral>(
	"JSBigIntLiteral",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
