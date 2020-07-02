/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSBindingIdentifier, JSClassHead, NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSClassDeclaration = NodeBaseWithComments & {
	type: "JSClassDeclaration";
	id: JSBindingIdentifier;
	meta: JSClassHead;
	abstract?: boolean;
	declare?: boolean;
};

export const jsClassDeclaration = createBuilder<JSClassDeclaration>(
	"JSClassDeclaration",
	{
		bindingKeys: {
			id: true,
		},
		visitorKeys: {
			id: true,
			meta: true,
		},
	},
);
