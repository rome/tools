/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSIndexedAccessType extends NodeBaseWithComments {
	readonly type: "TSIndexedAccessType";
	readonly objectType: AnyTSPrimary;
	readonly indexType: AnyTSPrimary;
}

export const tsIndexedAccessType = createBuilder<TSIndexedAccessType>(
	"TSIndexedAccessType",
	{
		bindingKeys: {},
		visitorKeys: {
			objectType: true,
			indexType: true,
		},
	},
);
