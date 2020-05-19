/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSExpression, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../utils";

export type JSImportCall = JSNodeBase & {
	type: "JSImportCall";
	argument: AnyJSExpression;
};

export const jsImportCall = createBuilder<JSImportCall>(
	"JSImportCall",
	{
		bindingKeys: {},
		visitorKeys: {
			argument: true,
		},
	},
);
