/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSTypeElement, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSObjectTypeAnnotation extends NodeBaseWithComments {
	readonly type: "TSObjectTypeAnnotation";
	readonly members: Array<AnyTSTypeElement>;
}

export const tsObjectTypeAnnotation = createBuilder<TSObjectTypeAnnotation>(
	"TSObjectTypeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {
			members: true,
		},
	},
);
