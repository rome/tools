/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSBindingIdentifier, JSFunctionHead, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../utils";

export type TSDeclareFunction = JSNodeBase & {
	type: "TSDeclareFunction";
	id: JSBindingIdentifier;
	head: JSFunctionHead;

	// For consistency with JSFunctionDeclaration, this can mostly be ignored
	declare?: boolean;
};

export const tsDeclareFunction = createBuilder<TSDeclareFunction>(
	"TSDeclareFunction",
	{
		bindingKeys: {
			id: true,
		},
		visitorKeys: {
			id: true,
			head: true,
		},
	},
);
