/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyTSEntityName,
	JSIdentifier,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSQualifiedName extends NodeBaseWithComments {
	readonly type: "TSQualifiedName";
	readonly left: AnyTSEntityName;
	readonly right: JSIdentifier;
}

export const tsQualifiedName = createBuilder<TSQualifiedName>(
	"TSQualifiedName",
	{
		bindingKeys: {},
		visitorKeys: {
			left: true,
			right: true,
		},
	},
);
