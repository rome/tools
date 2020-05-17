/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BindingIdentifier,
	BlockStatement,
	FunctionHead,
	JSNodeBase,
} from "../index";
import {createBuilder} from "../utils";

export type FunctionDeclaration = JSNodeBase & {
	type: "FunctionDeclaration";
	id: BindingIdentifier;
	declare?: boolean;
	head: FunctionHead;
	body: BlockStatement;
};

export const functionDeclaration = createBuilder<FunctionDeclaration>(
	"FunctionDeclaration",
	{
		bindingKeys: {
			id: true,
		},
		visitorKeys: {
			head: true,
			id: true,
			body: true,
		},
	},
);
