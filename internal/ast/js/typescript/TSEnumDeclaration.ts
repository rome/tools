/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSBindingIdentifier,
	NodeBaseWithComments,
	TSEnumMember,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSEnumDeclaration extends NodeBaseWithComments {
	readonly type: "TSEnumDeclaration";
	readonly id: JSBindingIdentifier;
	readonly const?: boolean;
	readonly members: Array<TSEnumMember>;
	readonly declare?: boolean;
}

export const tsEnumDeclaration = createBuilder<TSEnumDeclaration>(
	"TSEnumDeclaration",
	{
		bindingKeys: {
			id: true,
		},
		visitorKeys: {
			id: true,
			members: true,
		},
	},
);
