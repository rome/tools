/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSBindingIdentifier,
	JSFunctionHead,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSDeclareFunction extends NodeBaseWithComments {
	readonly type: "TSDeclareFunction";
	readonly id: JSBindingIdentifier;
	readonly head: JSFunctionHead;

	// For consistency with JSFunctionDeclaration, this can mostly be ignored
	readonly declare?: boolean;
}

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
