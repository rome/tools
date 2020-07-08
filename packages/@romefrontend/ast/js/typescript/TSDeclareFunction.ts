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
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type TSDeclareFunction = NodeBaseWithComments & {
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
