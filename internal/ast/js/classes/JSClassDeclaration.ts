/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSBindingIdentifier,
	JSClassHead,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSClassDeclaration extends NodeBaseWithComments {
	readonly type: "JSClassDeclaration";
	readonly id: JSBindingIdentifier;
	readonly meta: JSClassHead;
	readonly abstract?: boolean;
	readonly declare?: boolean;
}

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
