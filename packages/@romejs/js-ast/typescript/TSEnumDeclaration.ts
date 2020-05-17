/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {BindingIdentifier, JSNodeBase, TSEnumMember} from "../index";
import {createBuilder} from "../utils";

export type TSEnumDeclaration = JSNodeBase & {
	type: "TSEnumDeclaration";
	id: BindingIdentifier;
	const?: boolean;
	members: Array<TSEnumMember>;
	declare?: boolean;
};

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
